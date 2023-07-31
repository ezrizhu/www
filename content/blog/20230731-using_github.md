---
Title: Why I still Use Github
Description: With the usage of Github being controversial now, why am I still using it?
Tags: 
  - programming

---

For the past few months, I have seen a lot of people discussing how they’re
moving away from Github to other git hosting services like
[Sourcehut](https://sourcehut.org), [Gitea](https://gitea.com), and other hosted
or self-hosted alternatives. Mainly due to Github using public repositories to
train [Co-Pilot](https://github.com/features/copilot), and to have control over
their own code. I have thought a lot about this, and decided that, for the near
future, to stick with Github for most projects. Let me explain why.

I’m a student, my primary goal right now is to complete my studies and land a
job. To do that, I plan on doing well in school, and do other extracurricular
programming activities to stand out, making myself a more attractive applicant.
With Github being the most popular git hosting platform out there, I think
recruiters will look at my Github and see what I have been up to, along with
other metrics that they can see from my [Github
profile](https://github.com/ericzty).

Github has nice features like the activity graph that shows people how much I
contribute to projects on github, and the organizations I am associated with.
These information are all displayed on my Github profile, and allow recruiters
to have a crystal clear view of my contributions to various projects, and my own
projects.

One of the bigger reasons is the ease of use, my friends need not to learn how
to use all of Git to contribute to my projects. With in-line code editing, pull
requests, and issues, Github enables more novice programmers to contribute to
open source.

Github Actions is great, I use it to power almost, if not all of my CI/CD
pipelines. Considering most of my projects are public repositories, I basically
get unlimited free compute. I get to focus only on writing code, and not the
underlying testing and deployment infrastructure.

Github has a tightly integrated ecosystem that is constantly growing. I watched
Github add it’s kanban board, wiki, and discussions. All of them had made Github
a great platform to grow a developer team. Teams no longer have to find their
own wiki platform, or to pick between kanban solutions.
[Blueprint@Stevens](https://sitblueprint.com) chose Github because it allows us
to focus on getting our engineers working on our client’s projects, not to dwell
on the infrastructure, and how to connect them together.

However, there are cases where I will not use Github for, like schoolwork. I
usually have copilot enabled on my neovim setup, as it allows me to quickly
write boilerplate code to kick start projects. One day when I was working on a
schoolwork, I was in the middle of writing the school mandated Honor Pledge, “I
pledge my honor that…” and then, Co-Pilot had completed the rest, “I have abided
by the Stevens Honor System”. I was shocked, how did Co-Pilot know my schools
honor pledge? I searched on Github and found that students had been uploading
their schoolwork on Github, and Co-Pilot has been training off of them.

This isn’t great, and I really hope no lazy Stevens CS majors are reading this.
But I had realized that Github isn’t the place to put source code that you don’t
want the AI to give to other students. Although Github has now introduced an
option to deny them access to train their models using your code, them doing
that in the first place without any prior communication puts a bad taste in my
mouth.

Currently, all of my schoolwork are on sourcehut. For one of my freshman
classes, we had to work on problem sets for discrete structures in pairs. I
setup a sourcehut build script to automatically run pdflatex on the week’s
homework directory and upload the resulting pdf as an artifact. This did the
job, and I really enjoy using sourcehut due to it’s simplicity. However, I
understand that some people will have a harder time using it if they’re not
super familiar with git.

In the future, when I have a comfortable job, I will consider moving my projects
to other services. Although I will still be active on Github to contribute to
other open source projects. But for the foreseeable future, I’ll stick to
Github.
