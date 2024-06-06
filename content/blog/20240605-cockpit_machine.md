---
Title: hypervisor with cockpit-machine
Description: How to install cockpit-machine on a Debian machine, with network config and zfs.
Tags: 
  - linux
  - libvirt
  - hypervisor
  - zfs
  - networking

---

Sometimes, all you need is to deploy virtual machines on a server. You don't
want to care about the fancy cloudinit stuff, you don't care about users, and
groups. You just want a hypervisor that you can spin up VMs with and acccess the
console of via the web, while keeping your system relatively "clean".

In the past, I have always stuck with Proxmox, while that's a reasonable choice
for most use cases, especially ones involving giving others access to virtual
consoles and some other server management features. It is no longer fitting for
my purposes. I have outlined the reasons on [this post about the EVE Virtual
Environment](https://ezrizhu.com/blog/eve). However, EVE is yet to be done, and
I recently had the need to deploy a new hypervisor.

I have decided to go with a very simple, practical solution that doesn't litter
my system with things that is not super necessary.

For virtualization, I will be going with the QEMU backend, with libvirt.
However, all of that will be further abstracted by
[cockpit-machine](https://github.com/cockpit-project/cockpit-machines), a web
interface to interface with libvirt.

cockpit-machine is an add-on component of
[Cockpit](https://cockpit-project.org), which is developed by Red Hat. It is
also installed by default in some RHEL derivatives. It is very handy for
managing individual machines, however not too great for a fleet of machines. 

We will also be using zfs to setup a ZRAID array on our disks, and using a linux
bridge to create a bridged network.

## Setting up ZFS

```
zpool create tank raidz /dev/sdb /dev/sdd /dev/sda /dev/sdc
mkdir -p /data
zfs create -o mountpoint=/data tank/data
```

I am not super familiar with ZFS to be totally honest, and I just followed the
[Debian wiki on ZFS](https://wiki.debian.org/ZFS). I recommend doing further
reading before proceeding.

The first command creates a raidz pool with my 4 disks.
The second and third command creates the mountpoint for the data volume, and
mounting to that directory.

Later, we will put our VM's disks in the `/data` directory.

## Bridged Networking

You want your virtual machines to be sharing a connection with your server's
NIC. We can solve this easily with a bridge. You may also need the hypervisor to
do NAT, but we will not go over it in this post.

```
source /etc/network/interfaces.d/*

# The loopback network interface
auto lo
iface lo inet loopback

auto eno1
iface eno1 inet manual

auto br0
iface br0 inet static
        address 198.8.59.3/24
        gateway 198.8.59.1
        dns-nameservers 9.9.9.10 149.112.112.10
        dns-search thtp.bns.sh
        bridge-ports eno1
        bridge-stp off
        bridge-fd 0
```

Change the values accordingly, you can also setup an interface on a bond
connection. See below.

```
auto bond0
iface bond0 inet manual
        bond-slaves enp65s0 enp65s0d1
        bond-mode 802.3ad
```

[The Proxmox wiki page on network
configuration](https://pve.proxmox.com/wiki/Network_Configuration) has a lot of
documentation on other types of network configurations, such as a NAT.

## Installing Packages

First, we will install the packages that we will need.

```
apt update
apt install --no-install-recommends cockpit-machines libvirt-daemon-system qemu-system libvirt-clients libvirtd cockpit virt-install virtinst qemu-utils bridge-utils zfsutils-linux linux-headers-amd64
```

That should install all the packages that we will need. You should also have a
user account with sudo access created on your machine, since that is what we
will be using to access cockpit.

## Configuring Cockpit

You can also change the address/port that cockpit listens on via editing a file.
`/etc/systemd/system/cockpit.socket.d/listen.conf`

By default, it will be listening on port `9090`. You can read more about the
option to changing the listening parameter
[here](https://cockpit-project.org/guide/latest/listen).

> NOTE: The first line with an empty value is intentional. systemd allows
> multiple Listen directives to be declared in a single socket unit; an empty
> value in a drop-in file resets the list and thus disables the default port
> 9090 from the original unit. (from above linked documentation)

```
[Socket]
ListenStream=
ListenStream=192.168.1.130:443
```

Then, you will enable the cockpit socket so that it starts listening on the web
interface.

```
systemctl enable --now cockpit.socket
```

## Using Cockpit

After you land on the cockpit web interface, you will use your unix user's
username and password to login. Then, you can escalate your priviledge via the
button on the top bar, provided you have sudo permission.

You should also see the "Virtual machines" tab on the side bar, that is where
you will be setting up your virtual machines.

You can configure the storage pool to other locations besides the default ones.
And we have already setup a network bridge, so there is no reason to touch
libvirt's bridge options.
