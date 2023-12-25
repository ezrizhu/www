---
Title: How to Build Your Own ISP
Description: A guide on creating your own ISP.
Tags: 
  - networking
  - bgp
  - linux

---

This was written a while ago when I was back in high school, some information
may be outdated. If you spot any errors, please let me know and I will update
it.

This is also meant to be an overview of how the internet works, and how one
would setup a virtual BGP network on a linux machine. The exact commands and
configuration may vary depending on the setup.

## How the Internet Works

When you look up something on Google, is your computer connecting to the Cloud?
Are your favorite movies and TV shows stored up high in the clouds? Well, cloud
usually means 'someone else's computer'. In the case of Google, cloud are
Google's massive data centers. Your movie and TV shows are stored in physical
disks in enormous data centers that we call "cloud" today, and if you have an
iPhone with iCloud backup, your phone is backed up to servers in Apple's data
centers.

But how does your device connect to other people's computers? I'm sure you all
know it's with the internet, but how exactly does that work? Does your Internet
Service Provider (the company that gives your home internet or your phone's
wireless provider) have a line that goes directly from your home to Google's
data center? Well, not quite; the cable from your home or from your 4G/5G towers
connects to your ISP's data center. In the datacenter, they have connections to
all the other networks(i.e., other ISPs, companies, etc.) on the internet,
either directory, or indirectly via peering or internet eXchange points. I will
elaborate further on that.

Every connection on the internet originates from an IP address. For example,
your phone has an IP address, and your devices in your home that are connected
to the internet also have an IP address. An IP Address is like a street address.
When you connect to websites on the internet, it's almost like sending mail to
another person's house or a corporation's address. You will learn more about
that later aswell.

So when you look up something on Google, your device sends the request from its
IP address to Google's server at the IP address 172.253.115.113. It will go
through other devices, like your router, your ISP's router, and Google's
routers, until it reaches 172.253.115.113. Then Google's server will send the
response back to your device's IP address.

But how does the ISP know where 172.253.115.113 is? Does Google just have a
bunch of cables connecting its data center to other ISPs?

## How Your ISP Connects to The Internet

Well, Google does not have a bunch of wires each going to a different network
like your ISP. That would be too many wires, too costly, and too difficult to
maintain. Instead, Google connects to Internet eXchange Points (IXP), a location
where multiple networks connect. Companies usually have connections to one or
more Tier-1 network(s)(networks that can connect to every other network on the
internet with peering agreements.)

A peering agreement is an agreement between two networks to connect to reach a
more extensive network. Networks of similar size usually do this to eliminate
any need for payment as it is mutually beneficial. For smaller networks
connecting to a bigger network like a Tier-1 ISP, called "transit", and usually
incurs a cost for the smaller network receiving said "transit". Different
networks can have their own peering policy, such as open peering, which allows
any network to peer with them. There's also selective peering policy, where they
only allow some networks to peer with them, usually networks of similar size.
You learn learn more about internet peering on
[Wikipedia](https://en.wikipedia.org/wiki/Peering), and [internet
transit](https://en.wikipedia.org/wiki/Internet_transit) aswell.

What makes up a network? Two things, an ASN and IP prefix(s).

An ASN is an identifying number (like a license plate) that network routers use
with BGP to identify each other. Most large companies, like Google (AS15169),
Amazon (AS16509), and Meta (AS32934), all own an ASN. Every time you visit a
website, your ISP sends a packet from its ASN to the website's ISP's ASN. More
on [Wikipedia](https://en.wikipedia.org/wiki/Autonomous_system_(Internet)).

An IP Address is like a street address. When you connect to websites on the
internet, it's almost like sending mail to another person's house or a
corporation's address. You probably have a home IP address that comes from your
Internet Service Provider's pool of IP addresses, which we call an IP "block" or
IP "prefix.". You can learn more about IP Addresses on
[Wikipedia](https://en.wikipedia.org/wiki/IP_address).

How exactly do two networks connect? You probably can't just wire a cable
between two routers and expect it to magically work. This is where BGP (Border
Gateway Protocol) comes in: BGP is a protocol that helps facilitate
communications between two networks to exchange routes(IP prefixes). In short,
two networks use BGP to tell each other what IP addresses can be reached via
them. You can read more about BGP on
[Wikipedia](https://en.wikipedia.org/wiki/Border_Gateway_Protocol).

## IPv4, IPv6, Whats The Difference

There are two IP versions,
[IPv4](https://en.wikipedia.org/wiki/Internet_Protocol_version_4) and
[IPv6](https://en.wikipedia.org/wiki/IPv6). IPv4 is probably what you think of
when you think about an IP address: something like "192.168.0.1". IPv4 addresses
can range from 0.0.0.0 to 255.255.255.255, which leaves you with 4,294,967,296
possible IP addresses. As the internet became more and more popular, everyone's
home began to have its own IP address. Unfortunately, the "over-allocation" of
addresses has resulted in a shortage of IPv4 addresses.

This is where IPv6 comes in: an IPv6 address has more possible combinations due
to a longer address size and the use of hexadecimal characters. Here's an
example:

`2001:678:d3c:1234:abcd:2543:45cf:2b4c`

This address is clearly much longer than the previously mentioned IPv4 addresses
(i.e. 192.168.0.1) and as a result, can have many more combinations. Now, each
character can be a number (1-9) or a letter (a-f), making each character much
more meaningful than with IPv4. IPv6 addresses have so many combinations,
there's more addresses than individual grains of sand on Earth.

Humans have used all of the available IPv4 addresses. According to Google,
39.48% of users connecting to Google are using IPv6 as of May 14, 2022. And as
of Jan 2011, IANA (the authority on IP addresses) has ran out of IPv4 addresses
to distribute to companies like RIPE NCC. [More on IPv4 exhaustion
here](https://en.wikipedia.org/wiki/IPv4_address_exhaustion).

## You can Build Your Own ISP

That's correct! You can build your own network on the internet, along with your
own AS number and IP Prefix. You can connect your devices to it and browse the
internet with your own IP addresses.

Why would you build your own ISP?

* Learning: Well, it is a great way to learn computer networking, especially if
you're interested in getting a job in the tech field.
* Networking: There is also a whole community of hobbyist network operators out
there, allowing you to network with other people in the field and make new
friends.
* IPv6 Access: If your ISP does not provide IPv6, you can use your network's
IPv6 address to browse the web with IPv6. You can also offer your friends and
family with IP addresses to use.
* Cheaper IP Addresses: If you're planning on running a hosting company or
something that uses a lot of IP addresses, running your network would also drive
down the costs of renting individual IP addresses from another company.

## Getting an ASN and a Prefix

To run your own ISP, you will need a few things, mainly internet resources from
a LIR and internet transit from a provider.  
  
### An ASN and an IP Range

You can get an ASN and an IP range from a LIR, below are a few LIRs.

* [Inferno Communications](https://infernocomms.com/)
* [Glauca Digital](https://glauca.digital/)
* [OpenFactory](https://www.openfactory.net/)

### Server Provider

(Optional if you're in the [RIPE Service
Region](https://www.ripe.net/about-us/what-we-do/ripe-ncc-service-region))

If you're in the RIPE service region, you can ignore this step if you want to set
up your network from your home. But you can also set it up from a server
somewhere else like a cloud provider.  If you're not in the RIPE service region,
you will need to have a server in the RIPE region. Below is a list of server
providers in the RIPE region that also provides transit.  

* [Vultr](https://www.vultr.com/pricing)
* [VMHaus](https://vmhaus.com/plans)
  
### Transit Provider

You will need two networks to upstream your network to access the rest of the
internet and have justification for an ASN.

Below are a few **free** transit providers that I recommend. If you live in the
RIPE service region, you can run your whole network directly from your home and
connect to two of the transit providers below.  

* [Freetransit](https://freetransit.ch/)  
* [BGP Tunnel](https://bgptunnel.com/)
* [EzriCloud](https://as206628.net)  

### Setting Up BIRD on Your Server

Setting up your network  
Before we start, let's make sure we have everything ready.  
• A RIPE ASN  
• A RIPE IPv6 Prefix  
• Transit Providers (If you want to run your ISP from your home)  
• VPS Provider with transit (If you want to run your ISP from a server)  
  
For the following tutorial I will be using Debian 11 in a Vultr VPS with Vultr
as my upstream.  
  
Vultr (and some other transit providers) requires you to provide them with a LoA
(Letter of Authorization), allowing them to transit your IP Prefix and your ASN.
This can be done easily via [**_loa.tools_**](https://loa.tools), plugging in
your ASN's information and Vultr's (AS20473) information.  
  
We will use 65535 as our ASN in the following documentation and
2001:DB8:1234::/48 as our prefix.  
  
After you log into the server via ssh, we need to install bird2. Bird2 is a
software that communicates with other routers using BGP. (Note: If you have
firewall enabled, please allow 179/tcp)  
  
`ufw allow 179`  `apt install bird2`  We will also make a dummy interface where
we will route your prefix to. (Note: If you are setting this device up as a
router at your home, replace dummy0 with your LAN interface, and you can omit
the last line with 'pre-up') Edit \`/etc/network/interfaces\` and add the
following lines  
  
`iface dummy0 inet6 static address 2001:DB8:1234::/48 pre-up ip link add dummy0
type dummy`  
  
After bird2 is installed, we will edit its configration file at
\`/etc/bird/bird.conf\`  
  
```text
log syslog all;

router id 0.0.0.0;
# Replace 0.0.0.0 with your VPS's public IPv4 address

protocol device {
        scan time 5;
}

protocol direct {
        interface "dummy*";
        ipv6;
}

protocol static
{
        ipv6;
        route 2001:DB8:1234::/48 via yourVMsIPv6AddressHere;
}

protocol bgp vultr {
        description "Vultr";
        local yourVMsIPv6AddressHere as 65535;
        neighbor 2001:19f0:ffff::1 as 64515;
        multihop 2;
        password "bgpPassword";
        ipv6 {
                import all;
                export filter {
                        if net = 2001:DB8:1234::/48 then accept;
                };
        };
}
```

Do `birdc configure` to apply the changes. And do `systemctl enable --now bird2`
to ensure it is started and enabled.  
  
You can do `birdc show protocol all` to check the BGP session, you should see
the connection as Established.  To test using your own IPv6 address, do `ping -i
yourOwnIPv6Address google.com`

## Connect Your Local Devices to Your Network

Connecting your device to your network via a VPN.  
  
A VPN stands for Virtual Private Network, from Wikipedia "A virtual private
network extends a private network across a public network and enables users to
send and receive data across shared or public networks as if their computing
devices were directly connected to the private network." In short, it allows
your device to use the network connection from your VPS.  
  
We will be using the same machine from the last article.  
  
First we install our VPN server, wireguard. `apt install wireguard`  
Then we will create a public and private key pair.  
`cd /etc/wireguard wg genkey | tee privatekey | wg pubkey > publickey`  
This will create two files in the wireguard directory, publickey and privatekey.
Now we make a wireguard configuration at `/etc/wireguard/wg0.conf` And fill out
the configuration.  

```text
[Interface]
PrivateKey = privateKeyHere
Address = 2001:DB8:1234:a::/64
ListenPort = 7777 or any other port

[Peer]
PublicKey = Your device's wireguard publickey
AllowedIPs = 2001:DB8:1234:a:1::/80
```

Start and Enable that wireguard tunnel via `systemctl enable --now
wg-quick@wg0`
Enable forwarding by uncommenting `net.ipv6.conf.all.forwarding` in
`/etc/sysctl.conf` and setting it to 1 and restart the server.
Also allow port 7777 on your firewall.  
`ufw allow 7777`  
On your client, create a public and private key, and use the following config.  

```text
[Interface]
PrivateKey = ClientPrivatekey
Address = 2001:DB8:1234:a:1::/64

[Peer]
PublicKey = wireguard server's public key
AllowedIPs = ::/0
Endpoint = wireguard server public address:port
```

Start and Enable that wireguard tunnel via `systemctl enable --now wg-quick@wg0`

**And there you have it, your device connected to the internet from your own
network and your own address. Congrats!**
