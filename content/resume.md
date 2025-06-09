# Ezri Zhu's Resume

pdf version available [here](/files/Ezri_Tianyu_Zhu_Resume.pdf).

---

## Experience

### Portfolio Management Team Lead - [GSMIF](https://www.stevens.edu/news/stevens-launches-graduate-student-investment-fund)
(incoming) 2025-09 - 2025-12 @ Hoboken, NJ

### Recurser - [Recurse Center](https://www.recurse.com)
2025-05 - 2025-08 @ Brooklyn, NY

Currently in batch! Check back later.

### Visiting Research Intern - [Columbia University](https://www.engineering.columbia.edu/)
2024-05 - 2025-05 @ New York, NY

* Developed a monitoring system using **Prometheus** and **Grafana** to track
  the health of network muxes.
* Created a software tool in Golang to aggregate routing data from various
  sources, monitor specific routes, and expose metrics.
* Conducted experiments to identify and improve network performance bottlenecks
  in the [PEERING infrastructure](https://peering.ee.columbia.edu/).

### Head of Quant Development & Optimization - [SSMIF](https://www.stevens.edu/school-business/student-managed-investment-fund)

2025-01 - 2025-05 @ Hoboken, NJ

* Designed and implemented a centralized **financial database system** in **Clickhouse**
  that automatically collects and processes market data from multiple sources
  (FRED, Terminal, YFinance), including managing the back adjust price
  processing system, to power risk analytics and financial models.
* Led the development of a robust k3s **Kubernetes** stack featuring redundant
  databases that support our **financial modeling systems**, fullstack applications,
  and model creation/monitoring utilities, while implementing a comprehensive
  observability stack to ensure visibility of **metrics** and logs across all
  systems.

### Quantitative Analyst - [SSMIF](https://www.stevens.edu/school-business/student-managed-investment-fund)
2024-09 - 2024-12 @ Hoboken, NJ

* Improved existing equity screening interfaces, including AuthN, AuthZ, SSO,
  and implementing CI/CD.

### VP Technology - [Stevens Blueprint](https://sitblueprint.com/)
2023-07 - 2025-05 @ Hoboken, NJ

* Lead the technology team to develop and maintain internal tooling, compute
  infrastructure, and documentation to sustain our mission.
* Assisted other teams on their testing, **CI/CD pipelines** and hosting
  **infrastructure** to speed up development.

### Undergrad Research Asisstant - [PaSH Group](https://binpa.sh/)
2023-04 - Present @ Hoboken, NJ

* Debugged and modified complex C++ and C codebases to trace shell scripts in
  order to parallelize shell scripts.
* Utilized various **Linux APIs** via POSIX script to develop parts of our tool
  [try](https://github.com/binpash/try), as well as writing documentation and
  CI.
* Debugged various Linux Kernel features mostly involving namespaces and mounts
  for “try.”

### Volunteer Deputy-CTO @ [Fosshost](https://web.archive.org/web/20221121100228/https://fosshost.org/)
2021-10 - 2022-05 @ Remote

* Assisted leading TechOps in maintaining a fleet of **linux hypervisors** around the world, ensuring service availability.
* Onboarded and mentored new volunteers, including familiarizing them with the organization and its technologies.
* Designed, deployed, and maintained a monitoring system at scale that enhanced the team’s **observability**.

### Volunteer Techops @ [Fosshost](https://web.archive.org/web/20221121100228/https://fosshost.org/)
2020-10 - 2022-05 @ Remote

* Installed and configured support system to minimize support response time, and Gitlab for collaboration.
* Led in designing an identity and access system (RBAC), allowing for better control over system permissions.
* Deployed a global private network to facilitate secure communications between volunteers and internal infrastructure.

---

## Projects

### [wolfgirl.systems](https://wolfgirl.systems)
* Distributed managed NixOS public unix server cluster.

### [EVE](/projects/eve)
Golang, Postgres, mTLS, x506, CI/CD, Makefiles, Structured Logging
* A hypervisor suite for mid-scale server hosting deployments, with security and
  ease of usage in mind.
* Using mTLS to ensure secure connection between agent and main controller over
  any environment.
* RESTful API for users to control their resources, and for admins to manage
  users and create new machines.

### [EzriCloud](/projects/ezricloud)
Dual Stack Educational Network: BGP, RouterOS, Fastnetmon, Proxmox, Grafana, Prometheus, vLAN, IXP
* Provided reliable free hosting for 50+ students/open-source projects over a
  cluster of servers, and BGP transit for 10+ BGP networks.
* Developed a status checking website in Golang to minimize incident response
  time.
* Debugged various user networking issues, triaging incidents of various
  complexity ranging from user error to site wide DDoS attacks.

### [Personal Blog](https://ezrizhu.com)
Rust, Docker, CI/CD, responsive web design, high availability, SEO
* A personal website utilizing Rust’s concurrency and memory safety features.
  All requests are served directly from RAM.
* Used Github Actions, Watchtowers, Nginx with load balancing, resulting in a
  high availability setup.
* Maintaining an active blog about my journey in Computer Science.

### [Linux Systems Independent Research](https://ezrizhu.github.io/cs497/)
* Authored a comprehensive short book examining key differences between Linux
  distributions and their impact on software development, covering distribution
  building processes, cross-distribution compatibility, and practical
  reproduction guides for analysis

---

## Skills
**Programming**: Node.JS, Python, Golang, bash, POSIX shell, HTML/CSS, Javascript, Rust, BPF, C++, C, Java, Scheme, CI/CD tools, git, Postgres

**SRE**: NGINX, Apache, Docker, Debian, RHEL, libvirt/KVM, Kubernetes, Grafana, Prometheus, NixOS, Ansible, Puppet

**Computer Networking & Security**: BGP, Nftables, routerOS, EOS, IRR, RPKI, WireGuard, OpenVPN, MTR, Netcat, Metasploit, nmap, IPS

---

## Education
### B.S. in Computer Science @ [Stevens Institute of Technology](https://www.stevens.edu/school-engineering-science/departments/computer-science)
2022-09 - 2026-05 @ Hoboken NJ

**Relevant Coursework**: Data Structures, Discrete Structures, Algorithms, Statistics,
Computer Architecture, Systems Programming, Programming Languages, Operating
Systems, Systems Administration, Theory of Computation, Distributed Systems &
Cloud Computing, Automated Techniques for Security, Privacy & Reliability

---

Like what you're reading? [reach out!](/contact)
