---
Title: Network Latency and CPU
Description: It's not DNS, it's your CPU
Tags: 
  - networking

---

For my NYC network, I have been tunneling everything thru a
[wireguard](https://www.wireguard.com/) tunnel to
[Vultr](https://www.vultr.com/) for BGP.

Usually it is not this way, it should always be using my upstream's routers, but
currently they're down right now. So I am basically tunneling to Vultr and use
them as my BGP upstream.

But since I have made the switch, the latency to my servers has been
dramatically higher, and it didn't make a lot of sense.

```text
eric in ide0 in ~ took 11s
λ ping 1.1
PING 1.1 (1.0.0.1) 56(84) bytes of data.
64 bytes from 1.0.0.1: icmp_seq=1 ttl=56 time=22.5 ms
64 bytes from 1.0.0.1: icmp_seq=2 ttl=56 time=15.2 ms
64 bytes from 1.0.0.1: icmp_seq=4 ttl=56 time=107 ms
64 bytes from 1.0.0.1: icmp_seq=5 ttl=56 time=91.2 ms
64 bytes from 1.0.0.1: icmp_seq=6 ttl=56 time=56.6 ms
64 bytes from 1.0.0.1: icmp_seq=7 ttl=56 time=97.8 ms
64 bytes from 1.0.0.1: icmp_seq=8 ttl=56 time=77.6 ms
64 bytes from 1.0.0.1: icmp_seq=9 ttl=56 time=77.7 ms
64 bytes from 1.0.0.1: icmp_seq=10 ttl=56 time=131 ms
64 bytes from 1.0.0.1: icmp_seq=11 ttl=56 time=164 ms
64 bytes from 1.0.0.1: icmp_seq=12 ttl=56 time=166 ms
64 bytes from 1.0.0.1: icmp_seq=13 ttl=56 time=213 ms
64 bytes from 1.0.0.1: icmp_seq=14 ttl=56 time=218 ms
64 bytes from 1.0.0.1: icmp_seq=15 ttl=56 time=215 ms
64 bytes from 1.0.0.1: icmp_seq=16 ttl=56 time=422 ms
64 bytes from 1.0.0.1: icmp_seq=17 ttl=56 time=217 ms
64 bytes from 1.0.0.1: icmp_seq=18 ttl=56 time=273 ms
64 bytes from 1.0.0.1: icmp_seq=19 ttl=56 time=221 ms
64 bytes from 1.0.0.1: icmp_seq=21 ttl=56 time=117 ms
64 bytes from 1.0.0.1: icmp_seq=22 ttl=56 time=41.3 ms
64 bytes from 1.0.0.1: icmp_seq=24 ttl=56 time=13.4 ms
```

The ping was fluctuating from 22 to 221, which is very strange... My first thought
was that the network connection is just very unstable, but that shouldn't be the
case.

Pinging the Vultr server directly resulted in the following.

```text
rtt min/avg/max/mdev = 2.181/7.680/30.317/8.120 ms
```

Even adding in the round trip time, it should not be 200-400ms!

So the issue was probably on the Vultr server, and a ping to cloudflare did
result in some high uptick in latency.

```text
rtt min/avg/max/mdev = 2.430/33.384/138.564/40.807
```

I tried this in another machine on the vultr, and the latency to cloudflare was
consistently well, below 5ms.

So I thought, maybe it's the CPU, but the load average was only 0.2, on the 1
core router machine... But there's no other explanation...

I decided to do some controlled experiment, I turned off the wireguard tunnel,
ran a mtr, turned it on, then ran another mtr. Perhaps it *is* the CPU, and that
if I turned off the wireguard tunnel, the latency would decrease.

![screenshot of a screen showing good latency while wireguard is off, but up to
200ms when it's back on](/assets/img/blog/20230917-mtr.png)

Huh, that explains it! When the wireguard tunnel is off, the latency is pretty
good, but when it is back on and processing my traffic, the latency skyrockets,
even tho the load time is only .2 on the single-core machine.

Looking back at the Vultr plans, I was using the High Performance AMD cores, I
thought that those are probably fast enough, but it seems like it's just their
fancy wording for "burstable CPUs".

I decided to switch to their "CPU Optimized" compute instances, since it
looked like it can handle more CPU intensive workloads like routing all my
packets. There's also High Frequency, tho I am not sure how much that will help
here.

The switch-over was fairly simple, I converted that router's current IP to a
reserved floating IP so I don't have to change the IP address. I then took a
snapshot, but that took a while. After the snapshot finishes, I removed the
router VM to free up that reserved IP, created a new instance for the router,
re-assigned the IP address, restored the snapshot.

```text
Before:
rtt min/avg/max/mdev = 119.650/172.541/217.483/35.735 ms
After:
rtt min/avg/max/mdev = 5.686/6.738/7.449/0.584 ms
```

Wow, that's an improvement, so it seems like switching to a better instance
fixed my latency issue. 🎉
