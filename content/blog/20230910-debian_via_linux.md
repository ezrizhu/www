---
Title: Installing Debian From Any Linux Distro
Description: How I installed Debian Linux with an Arch live system.
Tags: 
  - linux

---

There are times where you have to install a Debian machine, but you don't have
access to a debian iso, or you don't already have a USB stick ready. This
happened recently to me when I was volunteering at
[NYCMesh](https://www.nycmesh.net/). We needed to install Debian on a machine,
but I only had my arch linux iso for some reason.

I remembered stumbling upon [this debian page on installing from a linux
machine](https://www.debian.org/releases/bookworm/amd64/apds03.en.html) (the
original link was broken since debian made new releases, but the content is
similar). This was exactly what I needed, but some commands had to be changed to
fit that system, which I will outline later.

This is similar to the install process of some more hardcore linux distros, like
arch, void, and gentoo. So because I had experience installing distros from
scratch, the process was pretty straightforward.

1. You format the disks and mount them
2. run your distro's bootstrap script on that mount (this sets up the package
   manager)
3. `chroot` into the disk
4. install base packages, like your linux image, headers, bootloader, etc...
5. setup things like your keyboard layout, locale, time, networking, drivers
6. install the bootloader
7. restart and pray that it all works

The debian guide I linked above contains pretty much everything that you'll
need. However, I ran into some issues trying to install the grub bootloader, for
my future convenience and whoever is out there that ran into the same issue as
me, I'll write down what I did.

It appears that grub needs to know where your boot drive is, but it was not able
to because your drives were not properly mapped in /dev. But you can fix it by
doing the following from [RescueLive Guide](https://wiki.debian.org/RescueLive)
before entering the `chroot`.

`for name in proc sys dev ; do mount --bind /$name /mnt/$name; done`

This command will mount proc, sys, dev to your chroot, so grub can read them
inside your new linux instance.

`grub-install` also had to be run with `--force` for some reason still
unbeknownst to me. If you have any idea of why this is the case, please let me
know.

I am still not super familiar with the process, and my memory of it is a big
foggy since I did this a few weeks ago, I have just been really busy and not
been able to write this down.

(I also ended up installing [Ventoy](https://www.ventoy.net) on my USB drive so
I can load up multiple linux images onto it)
