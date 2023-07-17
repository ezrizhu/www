---
Title: Try (1)
Description: Inspect a command's effects before modifying your live system. Try uses Linux's namespace (via unshare) and the overlayfs union filesystem.
---

I have been working on try with the rest of the [PaSH](https://binpa.sh) team.
`try` lets you run a command and inspect its effects before changing your live
system. `try` uses Linux's [namespaces (via
`unshare`)](https://docs.kernel.org/userspace-api/unshare.html) and the
[overlayfs](https://docs.kernel.org/filesystems/overlayfs.html) union
filesystem. [Link to repo](https://github.com/binpash/try)
