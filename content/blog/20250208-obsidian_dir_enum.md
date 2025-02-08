---
Title: Obsidian Publish Directory Enumeration
Description: Hidden Obsidian pages aren't truly hidden
Tags: 
  - obsidian
  - security

---

I have been using Obsidian for a while now. It is a great tool for organizing my life. My daily TODO lists, project boards, notes for school and research, and the occasional journal are all stored in Obsidian. At a certain point, I needed to share some notes with my collaborators, and that's where [Obsidian Publish](https://obsidian.md/publish) came in.

However, when the "show navigation" setting is off in publish options, users might assume their Obsidian pages are no longer indexable by others. This feature seems useful for sharing pages with specific individuals via direct links while preventing the general public from discovering those links. I initially thought it was a reasonable approach for sharing somewhat sensitive files, similar to unlisted videos on YouTube.

![A screenshot of the obsidian publish options, showing various options as 'display'](/assets/img/blog/20250208-obsidian_publish_options.png)

It appears that, similar to pages with navigation enabled, a web request to a published page still fetches a list of all published pages on that vault via `https://publish-01.obsidian.md/cache/id`, enabling a malicious user to enumerate all available pages.

Upon testing, it also appears that if you block the above URL, the page fails to load but a search box appears, allowing that malicious user to perform full-text search on the vault, even though the user has "show search bar" disabled.

This may be out of scope regarding whether Obsidian should hide these pages at the API level. However, I think it should be better documented in the Obsidian vault to prevent sensitive data leaks due to incorrect assumptions users may have about these toggles.

Furthermore, it appears that all UI options can be enabled client-side via JavaScript.

POC by [eva](https://kibty.town) You can find the original page, and the page after the clientside js change.
![proof of concept clientside js change](/assets/img/blog/20250208-obsidian_publish_poc_code.png)
![an obsidian publish page with no navigation options, just a plain page](/assets/img/blog/20250208-obsidian_publish_poc_orig.png)
![an obsidian publish page with a list of all files, mindmap, and a search bar](/assets/img/blog/20250208-obsidian_publish_poc_result.png)

### Response from Obsidian
> At this time, Publish isn't designed to secretly share content and that it's more aimed at publishing public content. For private sharing one can use password protection, although it is just all or nothing.
> I'll work on seeing if we can make it more apparent that just because the navigation is hidden, that the files are not any more secure.

In my opinion, Obsidian Publish would be a stronger choice if it could disallow file enumeration. Regardless, this limitation should be clearly documented so users won't make incorrect assumptions that could lead to potential data breaches. Currently, I am using [A Moon Server plugin](https://github.com/Dzoukr/MoonServerObsidianPlugin) on an instance hosted by a friend.
