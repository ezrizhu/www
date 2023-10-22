---
Title: VNC Proxying for the Cloud
Description: How I am proxying VNC for Eve
Tags: 
  - golang
  - hypervisor

---

I’m currently working on eve, a management toolkit for libvirt-based
virtualization servers. Think of it as openstack, but for small scale cloud
deployments. I am building it primarily for deployments like EricNet, small
scale, multi-siloed, and with user support. It features a management pane, and
it talks to hypervisor agents to interact with libvirt and other systems on each
hypervisor, like firewalling, backups, etc…

[Github link to Eve](https://github.com/BasedDevelopment/eve)
[Github link to Auto](https://github.com/BasedDevelopment/auto)

Today we are going to talk about proxying the connection for user consoles. As
with every cloud provider out there, we want to allow our users to access the
server console. In the case of QEMU, this is usually done with VNC.

In my case, giving users a direct connection to the hypervisor isn’t gonna cut
it, because not all hypervisors will have a public address, and we don’t want
people to brute force the VNC’s 8 character passwords.

Thankfully, we already have a software that serves as the bridge between the
user and the hypervisor, and that’s eve, the management pane. Auto is the name
of our hypervisor agent, that’s responsible for managing each hypervisor as
instructed by eve. Eve and Auto talk to each other via HTTPS via a mTLS
connection; auto is a webserver that’s listening for requests from Eve, and it
is listening with a TLS certificate signed by Eve, it also knows Eve’s cert
fingerprint. Eve also performs checks by verifying that Eve’s cert’s fingerprint
is correct and that it’s indeed signed by Eve CA.

Here, we will go over the process between the request first hitting the eve API,
and how we’re sending that to QEMU’s VNC.

First, the request will have to get accepted by our middleware, and usually we
do that with a token that’s issued to the user at login. And the user presents
that in the HTTP header, using Bearer Authentication. However, since VNC is
proxied via websocket, it does not have support for header field.

So we have added a way for the middleware to extract and authenticate the user’s
token.

```go
func getTokenFromQuery(w http.ResponseWriter, r *http.Request)
(tokens.Token, error) {
        tokenString := r.URL.Query().Get("token")
        if tokenString == "" {
                return tokens.Token{}, ErrBadToken
        }

        token, err := tokens.Parse(tokenString)
        if err != nil {
                return tokens.Token{}, ErrBadToken
        }

        return token, nil
}
```

Here, the request hits eve and gets sent to GetVMConsole, we’re using
`getUserVM` to check if we have the VM that the user is asking, and we get the
hypervisor and vm types. We then call the hypervisor’s websocket method with the
VM type’s UUID, and the rest of the request.

```go
func GetVMConsole(w http.ResponseWriter, r *http.Request) {
        hv, vm := getUserVM(w, r)
        if vm == nil {
                eUtil.WriteError(w, r, nil, http.StatusNotFound,
                "virtual machine not found")
                return
        }

        hv.Auto.WsReq(w, r, vm.ID.String())
}
```

Then, we construct the URL that we will be reverse proxying from auto, and
sending it off to auto’s WSProxy method.

```go
func (a *Auto) WsReq(w http.ResponseWriter, r *http.Request, domid string) {
        wsUrl, err := url.Parse(a.Url)
        if err != nil {
                return
        }
        wsUrl.Path = "/libvirt/domains/" + domid + "/console"
        a.WSProxy(wsUrl, w, r)
}
```

Now, we will be creating our https client from our TLS configurations, and
creating the reverse proxy. We’re also stripping the path and query that the
user sent us.

```go
func (a *Auto) WSProxy(wsUrl *url.URL, w http.ResponseWriter, r *http.Request) {
        tlsConfig := a.getTLSConfig()

        proxy := httputil.NewSingleHostReverseProxy(wsUrl)
        proxy.Transport = &http.Transport{
                TLSClientConfig: tlsConfig,
        }

        rr := r.Clone(r.Context())
        rr.URL.Path = ""
        rr.URL.RawQuery = ""

        proxy.ServeHTTP(w, rr)
}
```

Now we’re switching over to Auto.

Here, auto will first fetch and domain (libvirt’s way of calling a machine), and
using the `GetVMConsole` method to fetch the VNC port. Then, we’re stripping the
url PATH that we received from eve, since it won’t play nicely with the VNC
server. Lastly, we re-write the websocket request and reverse proxy that.

```go
func GetConsole(w http.ResponseWriter, r *http.Request) {
        domain, err := getDomain(r)
        if err != nil {
                eUtil.WriteError(w, r, err, http.StatusNotFound,
                "Invalid domain ID or can't be found")
                return
        }

        port, err := HV.GetVMConsole(domain)
        if err != nil {
                eUtil.WriteError(w, r, err, http.StatusInternalServerError,
                "Failed to get console port")
                return
        }

        // Since libvirt's vnc doesn't accept any path,
        // we will rewrite it to empty since proxyer will add it
        r.URL.Path = ""
        wsUrl := &url.URL{Scheme: "http", Host: "localhost:" + port}

        proxy := httputil.NewSingleHostReverseProxy(wsUrl)
        proxy.ServeHTTP(w, r)
}
```

And there we have it, the Auto instance reverse proxies the websocket connection
from the local QEMU connection. The Eve instance authenticates and reverse
proxies the websocket connection from the Auto instance.

I’m not sure if this is the industry standard way of doing this, but this method
works for me, and perhaps will be for other people.
