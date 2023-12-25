---
Title: August Server Upgrades (5 Hours of Dread)
Description: How a two hour Linux server upgrade turned into five hours.
Tags: 
  - linux

---

I run a small sized server infrastructure consisting of mainly three hypervisors
in California, and one hypervisor in NYC. Those servers are all owned and
maintained by me, with the occasional help of the helpful datacenter staff. You
can read more about EzriCloud [here](https://ezrizhu.com/projects/ezricloud)

My servers were awfully out of date, being two major versions behind on Proxmox,
and Debian. I have been ignoring the EOL warning as I thought I would have
finished writing my Proxmox Replacement [Eve
Suite](https://ezrizhu.com/projects/eve) soon. But I have came to the realization
that it will still take me a while, and that it’s not super safe being
vulnerable to all the vulnerabilities out there to Linux and Proxmox. That’s
where I have decided to upgrade all my hypervisors and other machines that runs
Linux. I also had to renumber my servers to a new IP range, so I decided to use
this opportunity to renumber as-well.

![Screenshot of Proxmox showing 1k+ days of uptime on my
servers](/assets/img/blog/20230814-uptime.png)

And here is where the dreaded night of upgrading and renumbering Proxmox came
in. I posted a maintenance window of two hours, and first started upgrading my
linux VMs as they won’t impact anyone besides the few BGP downstreams.

It wasn’t long after changing the repositories from Debian 10 to Debian 12 that
I ran into my first issue. Linux kernel refused to build because I had
wireguard-dkms and aufs-dkms installed. Wireguard was introduced to the Linux
kernel, and so the wireguard-dkms upgrade script will not run for the newest
debian kernel, so I had to remove that package. A similar issue happened with
aufs-dkms, although I don’t ever remember installing that in the first place.
After I removed those two packages, apt was able to successfully build my Linux
kernels and the initramfs.

This issue was also discussed in the [debian mailing
lists](https://groups.google.com/g/linux.debian.bugs.dist/c/mz2SZm62TAA)

Soon enough, another issue came up. Because I was jumping from Debian 10 to
Debian 12, libcrypt.so.1 was not able to open shared object file when upgrading.
After doing a lot of googling and kind of regret doing this at 12am, I stumbled
upon [this mailing list
post](https://groups.google.com/g/linux.debian.bugs.dist/c/3rZpOKvCh5E?pli=1)
Aha! Someone ran into the same issue as me, while jumping a version when
upgrading. I followed the instructions that someone posted in the thread;
manually downloading libcrypt1 and unpacking the files into /lib, and it
resolved my issue.

After painstakingly upgrading the route collector and other linux VMs, and the
clock is now at 0:22, I have started to shut down user VMs and prepare the three
hypervisors for upgrades.

After learning the lesson to not jump the upgrade, I have decided to first
change the repos to debian 11, do a full-upgrade, then change the repos to
debian 12 and do a full-upgrade. I’m also upgrading the machine one by one so
Proxmox doesn’t freak out about missing quorum.

First issue: box0.sn3 (a hypervisor I have in NYC) seems to be not booting back
up, I wasn’t able to see much text on the
[iDRAC](https://en.wikipedia.org/wiki/Dell_DRAC) since It was very blurry, and I
didn’t want to get the java applet to connect to the console since it was
running on iDRAC6 and did not have the HTML console. I attempted a cold reboot
via the iDRAC and the machine came back normally. Perhaps it was stuck in a
systemd shutdown service.

Then, box0.he2 (a dell r810 at California) was also not coming back up. I was
also not able to connect to that server’s iDRAC, I could not even ping the
iDRAC. That was very strange, I have never seen the iDRAC become unresponsive
like that. I phoned up the datacenter and asked them to go and reboot the
machine. The datacenter technician came back on the phone and told me that he
saw something very weird, that the LDC panel was lit, but blank, none of the
other lights were flashing, and that the machine was unresponsive to the button
press. That sounded very strange, however it does line up with iDRAC being
unresponsive to ICMP pings. I asked the kind technical to unplug and replug the
power cables to the server and see if that can restart the iDRAC. Thankfully
that brought the server back up, however…

The iDRAC came back online and was responding to pings, I thanked the datacenter
technical and hung up the phone. Although Linux is still not coming back up, I
logged in to iDRAC to investigate. “DIMM Unsupported……  Strike the F1 key to
continue, F2 to open system setup” What??? That’s strange, my RAM is supposed to
be fine. I asked a friend for the [link to the macOS iDRAC
viewer](https://github.com/scottjg/systemscope) and accessed the console. I
struck F1 and watched as Linux booted up normally, and that all my RAM showed up
in the system, although at a slower clock speed. I restarted the machine again
and the same prompt appeared, I struck F1 and accepted the slower clock speed.

![screenshot of the server console reporting unsupported
DIMMs](/assets/img/blog/20230814-dimm.png)

Although I would love to investigate why that was happening, but because I live
in NYC, and the server is in California. I do not have the energy or the money
to travel to California to see why that ancient r810 is acting up.

After Linux booted up and as Proxmox is attempting to spin the VMs back up, I
got “KVM not enabled”. What? KVM was enabled, I guessed that the CMOS battery
must’ve died and reset the BIOS, I also took the chance to double check that the
clock cycle of the RAM was correct in the BIOS, although it was indeed correct.
After enabling KVM, striking F1 to continue to boot process, I was able to
successfully bring box0he2 back up!

Great, now that all my hypervisors are up to date, time to renumber them. I had
done a bit of research beforehand on renumbering clustered Proxmox instances, so
it was fairly easy. I stopped the Proxmox damons, edited the appropriate
configuration files, incremented the version of the config files so it doesn’t
get overwritten on sync, and restarted the damons. Or so that’s how I wish it
went, I forgot to increment the version of the config files and Proxmox kept
overwriting the IP addresses of it’s nodes. But eventually I figured it out.

At this point, all the machines are now up to date and renumbered, but for some
reason unbeknownst to me yet my downstream’s IPs were not being announced… It’d
appears that my route collector took too long, and Hurricane Electric had
already done their filter refresh. I emailed them and they were able to quickly
refresh my filter.

The time is now 4:52 am, almost three hours past the expected end of the
maintenance window. I head to sleep as I need to pack up to prepare to move to
my new apartment tomorrow. Hopefully this is the last time I upgrade Proxmox,
this experience was a great motivation to me to finish writing [Eve
Suite](https://ezrizhu.com/projects/eve).

![screenshot of me notifying my users the status of this server
upgrade](/assets/img/blog/20230814-screenshot.png)
