---
Title: Strange M2 UTM Crypto Bug
Description: UTM emulation bug leads to failed debian installation.
Tags: 
  - apple
  - debugging

---

So in Operating Systems class, we had to install Debian, Arch, and OpenBSD for
our first homework to get familiar with UNIX systems.

My friend and some other classmates had issues with the Debian installer failing
at the tasksel stage where they were instructed to install Gnome.

This was weird; the tasks step should usually not fail. I tried reproducing the
issue on my VM, but it was installed successfully.

One common demonstrator within the failed installations was all using the UTM
virtualization software on an M2 Mac machine.

I asked my friend to install a bare-bone Debian machine with nothing selected at
the tassel stage. So, we will have a working system to debug this issue.

After he installed Debian, I asked him to run the apt install
task-gnome-desktop. And that failed with the following.

```text
req: Error generating RSA key
40073529347F0000:error:01800079:bignum
routines:ossl_bn_rsa_filps186_4_derive_prime:no prime candidate:../crypto/bn/bn_rsa_flips186_4.c:353: pkg: error processing package
ssl-cert (--configure): Installed ssl-cert package post-installation script subprocess returned error exit status 1
```

That's quite weird. It was related to openssl generating something for ssl-cert.
But do we really need ssl-cert? The answer is no. It was only a recommended
package for task-gnome-desktop. We were able to do a workaround with
`apt install --no-install-recommends task-gnome-desktop`

Great, that's fine, but we would need a machine with crypto capabilities for
this class... So, I dug into it further.

I tried to isolate the issue by asking him to run openssl genrsa -out aaa.key
2048

That also failed with the "no prime candidate" error. Okay, we know how to
reproduce this error without running some apt post-run hook...

The issue seems to be related to [This UTM issue](https://github.com/utmapp/UTM/issues/4924).

I then asked him to set `sysctl -w kernel.printk=8` and see if running the
openssl command would generate any kernel debug messages - It did not.

Further debugging showed that it could not connect to any TLS website, with both
ECDSA and RSA certifications.

Then I had an idea: if the issue is with the CPU of the guest machine, what if
we switched it. The PDF the professor gave us told us to use the CPU "Enables
all features supported by the accelerator in the current host (max)," which
could be causing the issue with the crypto part of the CPU.

I asked him to switch to the qemu64 CPU, and the openssl command worked. He also
completed a Debian system installation with the Gnome DE. The professor also
updated the PDF's macOS instructions to include the CPU that works.

We should also be able to find the specific flags that we could toggle to
perhaps get the original CPU to work, since it was apparently faster.

I also added a comment to that [upstream
issue](https://github.com/utmapp/UTM/issues/4924#issuecomment-1903115494).
