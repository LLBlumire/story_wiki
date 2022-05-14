<div align="center">

![](resources/Logo.png)

# Story Wiki

**Spoiler Free Wiki Software**

[![Join Discord](https://img.shields.io/discord/966025436222095360?label=discord&style=for-the-badge)](https://discord.gg/wNpPRkqBzw)

</div>

# StoryWiki

StoryWiki is wiki software with multiverses and time travel. It is designed to be used to document stories.

Time travel is a system by which a user is able to view the wiki at various release points, essentially allowing them to filter out spoilers for content they haven’t seen yet. Removing the annoying problem of going to check a characters name on a wiki, and seeing “Albus Dumbledore ***was***” and spoiling the character is going to die.

Multiverses allow you to seperate out seperate canons from each other, for example you may have the book canon, with releases “Philosophers Stone” to “Deathly Hallows”, and you may have the movie canon which adapts from teh books with releases “Philosophers Stone” to “Deathly Hallows Part 2”.

## File Structure

You have a high degree of flexibility over your file structure, everything is served relative to your site-root, which by default is `/`, your site root (beneath `/site-root/` should serve the following: 

```bash
# REQUIRED TO SERVE WEBSITE
/site-root/index.wasm
/site-root/index.js
/site-root/index.html
/site-root/manifest.toml
/site-root/style.css

# REQUIRED TO CONFIGURE FAVICON
/site-root/apple-touch-icon.png
/site-root/favicon.ico
/site-root/icon-192.png
/site-root/icon-512.png
/site-root/icon.svg
/site-root/manifest.webmanifest

# REQUIRED FONTS IF YOU USE THE DEFAULT STYLE
/site-root/charter_bold_italic.woff2
/site-root/charter_bold.woff2
/site-root/charter_italic.woff2
/site-root/charter_regular.woff2
```

Your server should respond `/site-root/index.html` to all requests. Allow the site to handle serving unknown routes, and routing based on the `manifest.toml`.

## Manifest

To build your StoryWiki website, the most important thing you will need is your manifest. This configures the website, sets up your continuities, and your releases.

You don’t need to configure multiple continuities, or releases, if you want to just you one, or neither of these features you can.

If you only configure one continuity, the continuity switcher wont show, and your URL paths will not be prefixed with `/continuity/`.

If you only configure one release per continuity, the release switcher wont show, if ***any*** continuity has more than one release configured, then ***all*** continuities will show the release switcher.

The manifest is written in **[Tom's Obvious, Minimal Language](https://toml.io/en/).**

Your manifest must configure the following global setting:

```html
title = "My Wiki
unknown_category = "Uncategorised"
```

`title` is ***required*** and will configure the title of the website in the tab bar and in the nav bar.

`unknown_category` is *optional* and will configure the name of the category pages without a specified category are placed into.

Then, for every continuity on the website (at least one), you must configure it as follows:

```html
[[continuities]]
reference_name = "books"
display_name = "Books"
url_prefix = "books"
prefix = "b"
```

`reference_name` is ***required*** and is used to reference this continuity elsewhere in the manifest.

`display_name` is *optional* and is used for the title of the page and the continuity picker. If it is not supplied then a title case version of the `reference_name` will be used.

`url_prefix` is *optional* and is the path used in the URL to specify the continuity, if more than one continuity has been configured for this StoryWiki. If it is not required the `reference_name` is used.

`prefix` is *optional* and is a prefix that must be used for the `reference_name` of all pages beneath. If it is not specified, a unique prefix is taken from the reference name, such that if you had continuities with the referance names `light_novel`, `tv_series`, `movies`, and `manga`, you would have the prefixes `l`, `t`, `mo`, and `ma`.

Once you have specified your continuities, you will need to go on to specify at least one release for each continuity, use the `reference_name` for the continuity the release is on where `books` has been used below:

```html
[[releases.books]]
reference_name = "b1"
display_name = "Book 1"
begins_group = "Original Trilogy"
```

`reference_name` is ***required*** and is used to reference the release in conditional selectors, such that `o-{reference_name}` will show only if the user marks they have observed the release, and `x-{reference_name}` will show only if the user does not mark they have observed the release (i.e. they have excluded it.

`display_name` is *optional* and is used in the release picker to mark which releases have been observed. If it is not specified, a title case version of the `reference_name` will be used

`begins_group` is *optional* and is used to group together releases in the release picker. If it is not specified the release will be in the same group as the previous release. If it is specified it will create a new release. If it is specified as an empty string, i.e. `begins_group = ""`, then it will clear the group and place the release at a root level (the default before any `begins_group` is specifeid).

```html
[[pages.books]]
page_url = "john_butler"
resource_path = "path/to/resource/john_butler.md"
display_name = "John Butler"
show_cond = ["o-b3"]
keywords = "poet soldier king"
keywords_cond = [
    { keywords = "tinker tailor", cond = ["o-b4"] },
    { keywords = "spy", cond = ["o-b5"] }
]
title_peers = ["Johan", "John", "Butler"]
title_peers_cond = [
    { peer = "poet of kings", cond = ["o-b5"] }
]
categories = ["Characters", "Kingsmen"]
categories_cond = [
    { category = "Dead Characters", cond = ["o-b7"] }
]
```

`page_url` is ***required*** and is the URL that the page will be visible from, such that `/url_prefix/page/page_url` will be a complete path on a StoryWiki that has multiple continuities, and `/page/page_url` being a complete path on a StoryWiki with one continuity.

`resource_path` is ***required*** and is the path from which the user will download the content of the page, your site root will be prefixed to this (i.e. the above would be `/site-root/path/to/resource/john_butler.md`.

`display_name` is *optional* and will be used in the pages title. If not specified this will be a title case version of the `page_url`.

`show_cond` is *optional* is a list of conditions that must be true in order for the page to exist on the website. e.g. `show_cond = ["o-b3", "x-b5"]` will mean the page exists so long as the user has read book 3, but has not yet read book 5. If it is not specified the page will show unconditionally.

`keywords` is *optional* and is a list of keywords that the page should show up in when searching. The keywords will automatically have appended to it the pages display name. If they are not specified the keywords will only be the `display_name`.

`keywords_cond` is *optional* and *are* conditional keywords that should only apply if a user has or has not observed certain releases. If they are not specified there will be no conditional keywords.

`title_peers` is *optional* and are a list of strings that should redirect a user from the search page directly to the page if they are searched. The `display_name` of the website automatically behaves as a title peer. If they are not specified there will be no title peers (other than the `display_name`).

`title_peers_cond` is *optional* and **are conditional title peers that should only apply if a user has or has not observed certain releases.  If they are not specified there will be no conditional title peers.

`categories` is *optional* and are a list of categories the page should appear in on the category browser. If not specified the global `unknown_category` will be used.

`categories_cond` is *optional* and are categories that should only apply if a user has or has not observed certain releases. If it is not set there will be no conditional categories.

## Pages

The pages of your site will be served from the files specified in your manifest. These files use markdown syntax with a few extensions to make building your pages a breeze.

First of all, there are two important high level block structures you must be aware of, article and aside. These specify the main text of your page, and the infobox sidebar content.

Inside each of those, you may use any standard commonmark markdown syntax.

[https://commonmark.org/help/tutorial/](https://commonmark.org/help/tutorial/)

In addition, you can use strikethrough `~~like this~~` and tables

```html
| like    |    this    |
|:--------|:----------:|
| a       |     b      |
| c       |     d      |
```

You can also create headerless tables, like this


```html
<headerless-table>

|         |            |
|:--------|:----------:|
| a       |     b      |
| c       |     d      |

</headerless-table>
```

In addition, the power of releases in StoryWiki is available here, you can use custom tags to configure the visibility of elements on your page to show based on the releases a person has said they have observed. 

```html
<o-{RELEASE_REFERENCE}>
This text will show if you have observed {CONTINUITY_PREFIX}{RELEASE_REFERENCE}.
</o-{RELEASE_REFERENCE}>

<x-{RELEASE_REFERENCE}>
This will NOT show if you have observed {CONTINUITY_PREFIX}{RELEASE_REFERENCE}.
</x-{RELEASE_REFERENCE}>
```

For example, if your book continuity was `b`, and had releases `b1` to `b7`, and you might say

```html
<o-b3>This text will show if you have read book 3.</o-b3>

<x-b5>This text will only show until you have read book 5!</x-b5>
```

In addition to this, you can use snippets to load content from a specific path, allowing you to create content that is reused across different continuities.

```html
<include-snippet data-path="/site-root/path/to/snippet.md" />
```

By default, tags from a continuity you are not presently browsing are ignored, this allows you to write a snippet like

```html
<o-b3><o-m4>Shown after book 3 OR movie 4</o-m4></o-b3>
```

To put content in a snippet that has restriction control based on the continuity the snippet is loaded in. Inside the book continuity, it will look like 

```html
<o-b3>Shown after book 3 OR movie 4</o-b3>
```

and inside the movie continuity, it will look like

```html
<o-m4>Shown after book 3 OR movie 4</o-m4>
```

If you want content to react to a users observed releases in a continuity other than the one they are browsing in (this is not usually recommended), then you can use `<oo-...>` and `<xx-...>`. This would mean that you can write

```html
<oo-b3><oo-m4>Shown after book 3 AND movie 4</oo-m4></oo-b3>
```