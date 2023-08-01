---
Title: Hello IndieWeb!
Description: My website now supports some IndieWeb protocols!
Tags: 
  - meta
  - indieweb
  - webdev

---

Short update! My website now supports the following
[IndieWeb](https://indieweb.org/) protocols.

* [h-card](https://microformats.org/wiki/h-card)
* [h-entry](https://microformats.org/wiki/h-entry)
* [RelMeAuth](https://microformats.org/wiki/RelMeAuth)
* [webmention](https://en.wikipedia.org/wiki/Webmention)

The `h-card` and `h-entry` are just some machine readable code about my website
and my blog entries. They're both implemented as hidden `div`s.

RelMeAuth is just a `rel=me` email link so services can discover my email as a
authentication method.

For webmention, I wrote a simple [GoLang](https://go.dev/) API that accepts
webmention query and sends them to me as a discord webhook. The source can be
found [here](https://github.com/ericzty/dismention)

This website is my home, so I thought it'd be nice if it is somewhat connected with
the rest of the IndieWeb blogging scene.
