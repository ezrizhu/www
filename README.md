# www

This is the repo for [my website](https://ericz.me).

I took a lot of inspirations from [Xe's website](https://xeiaso.net/talks/how-my-website-works), I also wanted to learn rust and remake my website, so I decided to do this.

On startup, the website first compile all of the markdown files into html, minify css files, and store them in ram.

Then on http request, will template the page, concat the relevant css files, then shoot them over to the webclient.

css wise, I'm using [purecss](https://purecss.io/).

I don't use js.

In the future, the website will do more dynamic-ey things, perhaps a comment section, rss aggregator for friends' blogs, webmention, or anything else might be fun to add.

## CICD

![a graph describing how CICD is done with this website](https://not-a.link/4AT2e5E.png)
