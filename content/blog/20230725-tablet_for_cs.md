---
Title: You Only Need a Tablet for Computer Science
Description: How do I complete my computer science coursework with just an iPad? Anyone - and yes, anyone - with a modern-day tablet can do the same.
Tags: 
  - ipad
  - workflow
  - programming

---

Last May I finished the first year of my computer science degree. I was able to
get away with only carrying my iPad around campus, and being known as the kid
that uses the iPad for computer science.

## How I pulled it off

I have an ordinary iPad Air (Fifth Gen), it runs stock iPadOS, and has the M1
chip with cellular data. I also have the Magic Keyboard and an Apple Pencil with
it. I run [EricNet](https://as206628.net), an infrastructure and networking
project consisting of a few colocated servers in California and New York City,
running on my own ISP. In my cluster of hypervisors, the most important machine
is my workstation virtual machine (VM). The workstation VM is a simple Debian
install with no window manager or desktop environment, and it has
`openssh-server`, `tmux`, `neovim`, and a few development tools like the Rust
and Golang toolchain, and `git` installed onto it.

As you may have guessed, I do practically all my work on this workstation VM. I
think I first got the idea from trying out [Coder.com](https://coder.com/) a few
years ago. I started because I wanted to be able to switch seamlessly between
working on my laptop to my PC back in high school. Thanks to the power of SSH
and `tmux`, I was able to have a persistent development environment in the
“Cloud” that follows me wherever I go. All I must do was to `ssh` and `tmux attach
-t <name>`. It was a perfect workflow for me since `neovim` is my text editor of
choice, and `tmux` fits very well into the workflow.

**Moving between devices was so smooth that I was able to map my thought
processes into `tmux` panes and windows, and then resume exactly where I picked
off without having to think about what I was doing before. This advantage
allowed me to easily switch between different projects, significantly increasing
my time to resume working on projects.**

With this setup, I was able to attend classes and programming labs around campus
with just the iPad that I was already carrying for my math notes.I was also able
to work anywhere with a cellular connection — mostly on a bus, or on a train —
without having to deal with a throttled mobile hotspot connection or search for
public WiFi.

Quick reminder for those that are not super familiar:

* SSH (Secure Shell) allows me to access my server’s command line via a secure
  connection.
* tmux (terminal multiplexer) allows me to have multiple workspaces, windows,
  and split my screen. With that, I can work on multiple tasks at the same time,
  with my own arrangement of “windows.” Think of it as Apple’s Stage Manager, or
  [a tiling window manager](bspwm). Tmux sessions are always detached from your
  SSH session, meaning if you disconnect from the server, what was on those tmux
  windows will still be running in the background. With tmux, I can easily
  disconnect from the server, or switch to a different window, and trust that my
  commands or my editor will run uninterrupted. This is incredibly neat as
  sometimes the internet connection can be spotty when I am on a train.
* Neovim is a fork of vim, a popular terminal-based text editor. It’s a feature
  packed command line based editor that allows you to navigate and edit a file
  with just keyboard shortcuts. Although mouse pointer is supported if you
  prefer that. Personally, I do all of my file editing with the keyboard, never
  having to lift my hand off of the keyboard to reach for the trackpad or mouse.
  I find the keyboard-only nature of my workflow also makes me a more efficient
  programmer as neovim provides very powerful keyboard shortcuts for navigating
  a file, or making bulk changes.
* RDP and VLC are protocols to access a computer’s graphical interface, think of
  it as SSH but for GUIs. Although I don’t use them much, they’re especially
  useful for people that need to use graphical applications for their work on
  the development machine.

Now, you may be thinking: “What about graphical applications that can’t work
with SSH?” I decided to use a separate VM (to keep my main VM light), with
`xfce4` and `xrdp`, and I used Microsoft’s [Remote Desktop
App](https://apps.apple.com/us/app/remote-desktop-mobile/id714464092) to connect
to it from the iPad. I tried VNC at first, but the experience was extremely
undesirable as none of the clients worked well with the servers that I had
tried. The fact that VNC only supports passwords up to eight characters also
made it not a great option for me.

![Screenshot of the source code of this website opened with neovim and tmux on an
iPad](/assets/img/blog/20230725-ipad-screenshot.png)

Back to the iPad, I use [Termius](https://termius.com/) for SSH and SFTP
(transferring files over SSH) access to my server. Termius also has support for
port forwarding, cross-device syncing, and agent forwarding. One of the coolest
feature that Termius has is it’s ability to derive a ssh key from my iPad’s
[Secure Enclave
Processor](https://support.apple.com/guide/security/secure-enclave-sec59b0b31ff/web),
a physical coprocessor used for secure cryptographic functions of the iPad. I
trust this Secure Enclave due to Apple’s solid engineering on platform security,
which you can read more
[here](https://support.apple.com/guide/security/welcome/web). At the touch of
fingerprint-to-sensor, I could log into any of my servers, nearly instantly.
This is another benefit of using Apple’s Secure Enclave; The master key is
derived from your fingerprint or other biometrics, like Face ID.

For note taking, I use [GoodNotes](https://www.goodnotes.com/), as it was
recommended to me by many of my friends and I find it very enjoyable to use. I
have a [wireguard](https://www.wireguard.com/) to tunnel me into my internal
network whenever I have to access something sensitive. I also have
[Procreate](https://procreate.com/) and the [Affinity
Suite](https://affinity.serif.com/en-us/) for whenever I am feeling creative. As
of recently, I have also begun using [iA Writer](https://ia.net/writer) for
writing blog posts. I find iA Writer very pleasant to use, as it allows me to
focus on writing my blog posts in a way which I have never been able to before.

## How you can do the same

You can do the same! Any tablet works — a friend that deals in systems
programming and programming languages works off of their Samsung Galaxy Tab. On
iPadOS, Termius is my top choice due to its aforementioned versatility. There
are also free VNC and RDP clients on the App Store, although I find Microsoft’s
Remote Desktop app to be the most convenient for me. On an android tablet Termux
is a very popular terminal emulator and can bring you into a Linux environment;
meanwhile on iOS, there’s iSH. Besides the traditional `ssh`,
[Mosh](https://mosh.org/) is a roaming friendly ssh replacement for those that
wants a more responsive ssh session when the connection is not the best.

If you don’t want to work from your Linux environment on your Android tablet, or
if you are working off of an iPad, you probably need another server to work
from. Luckily companies are handing out servers like it’s nothing.

* [DigitalOcean](https://www.digitalocean.com/) was my first server provider.
  They have always been extremely reliable and user friendly. They offer $200
  credit for a whole year with the [Github Student Developer
  Pack](https://education.github.com/pack)!
* [Vultr](https://www.vultr.com/) is a DigitalOcean alternative that currently
  offers $250 credit for a few months. I currently use them to power parts of my
  network via their BGP features.
* [AWS](https://aws.amazon.com/), [GCP](https://cloud.google.com/),
  [Oracle](https://www.oracle.com/cloud/) all offer low-spec free tier servers.
* Tons of other server providers offer decent compute servers at competitive
  prices, like [Hetzner Cloud](https://www.hetzner.com) and
  [Scaleway](https://www.scaleway.com/en/).

If you need to do any hardware emulation, [QEMU](https://www.qemu.org/) is great
for that purpose.

During last year, I helped another classmate setup this environment on his iPad
alongside a server from my network. He was able to also learn Linux at the same
time, and never looked back!

I have affiliate links with DigitalOcean and Vultr for some free credits if you
don’t have the Github Students Pack, feel free to reach out if you’re
interested.

Next year, check back in and see how I am going with just a Steam Deck!

![A picture of a Steam Deck with neovim opened on a racket
file](/assets/img/blog/20230725-steamdeck.jpg)

*This article was edited by [Jack Dorland](https://jackdor.land/). Thank you!*
