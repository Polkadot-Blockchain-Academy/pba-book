# Contributor Guide

Thank you for interest in contributing to the Academy! ✨.

Before anything else, please read our [Code of Conduct](CODE-OF-CONDUCT.md) to understand our community's guidelines.

This guide is to help Academy contributors understand how all materials contained this repository are structured, how to interact with and modify them.
Multiple tools are provided for contributors to make slideshows, leader-guided workshops, and self-directed activities.

## Installation

The Academy is Rust _heavy_ and as such, you need to [install rust](https://www.rust-lang.org/tools/install) before anything else here.

In order to `make` your life easy 😉, there is a set of tasks that use [`cargo make`](https://sagiegurari.github.io/cargo-make/#overview).

With [`cargo make`](https://sagiegurari.github.io/cargo-make/#installation) installed, you can list all tasks included to facilitate further installation, building, serving, formatting, and more with:

```sh
# Run from the top-level working dir of this repo
makers --list-all-steps
```

The tasks should be self-explanatory, if they are not - please file an issue to help us make them better.

```sh
# Install all dependencies
makers i
```

<details>
<summary>(Not advised) Manual Install</summary>

You may opt out of `cargo make` described, but minimally will need to hav:

### Book - `mdBook`

- [Install of `mdBook`](https://rust-lang.github.io/mdBook/guide/installation.html)

Serve the book offline with:

```sh
# Run from the working dir of this repo
mdbook serve --open
```

### Slides and Tooling - `node`

Use [nvm](https://github.com/nvm-sh/nvm#installing-and-updating) to install and manage multiple node versions.
With `nvm` installed, from the academy top level dir:

```sh
# Ensure you have the right node
nvm i
# For yarn, you need to enable some node features
corepack enable
# Install Dependencies
yarn
# Run a slide server watching for file changes
yarn start
```

This should open a new browser tab with a simple listing of all slide decks to choose from.

### Embedded Slides

At this time, there is a "hack" to get the slides embedded in the book, where the static HTML assets from a slides build are coppiced _into_ the book so that they function in an `iframe`.
See the `Makefile.toml` for `[tasks.serve]` to see the commands required to manually get this working.
Again, it's much more convenient to use `cargo make` here vs. manually running this!

</details>

## Content Design

**The academy is focused on _practical application_ of web3 concepts we cover, more than simply understanding.**

### Organization

The entirety of the book, including assets (images, code, etc.) needed in this book lives in `./content/*`.
The directory structure is as follows:

```sh
content
├── <module>
├── index.md                  # Student facing module overview
├── faculty-guide.md          # Faculty facing guide on running this module
├── README.md -> index.md     # Soft link `ln -s index.md README.md` - for Github web reading
│  ├── <lesson>               # Lecture related, has slides
│  │  ├── img
│  │  │  ├── <media>          # png, gif, mp4, jpg, etc. used in *this lesson*
│  │  │  ├── ...
│  │  ├── page.md             # Typically a stub file for embedding `slides.md`
│  │  └── slides.md           # A `reveal-md` formatted document
│  ├── _materials             # Workshop, Exercise, or Activity related
│  │  ├── img
│  │  │  ├── <media>          # png, gif, mp4, jpg, etc. used in *this lesson*
│  │  │  ├── ...
│  │  ├── <material>.md       # Student facing instructions on some material
.  .  .   ...
```

- `<module>/README.md` - **_required_** soft link to `index.md`.
- `<module>/index.md` - **_required_** book page, **_must_** be listed in `SUMMARY.md`.
- `<module>/faculty-guide.md` - **_required_** page _not_ used in the book, **_must NOT_** be listed in `SUMMARY.md`.
- `<module>/<lesson>/page.md` - **_required_** book page, **_must_** be listed in `SUMMARY.md`.
- `<module>/<lesson>/slides.md` - **_optional_** slides, embedded into a `page.md`, **_must_** be embedded into `page.md`if slides are used.
- `<module>/<lesson>/img` - **_optional_** directory with media used in slides or pages in _this lesson_.
- `<module>/<lesson>/_materials` - **_optional_** directory with inclusions referenced in slides or pages

## Development Workflow

Typically, most work for lessons centers on the development of slides.
The pages they are embedded into are _primarily_ static stubs to host the slides within.
Workshop and Activity pages are an exception, where they do not usually have slides associated, or need more information outside slides.
Viewing the rendered markdown of slides is more important than when iterating on pages, in practice.

### Working on Slides with `reveal-md`

Slides include primarily lecture materials used to present in class, and those slides must contain `Notes:` sections with detailed **student facing** information about what is covered on a slide, _not only_ speaker-facing notes!
Typically the slide notes should embed all the references, resources, and further considerations for students to have as a resource during and after class.

To view and edit slides (only) in **watching** mode (updates immediately on changes to any file changes in the content):

```sh
# WATCHING server for slides only
makers serve-slides
# Or simply:
yarn s
```

See the [Using this Book](./how-to/page.md) page for more details on `reveal.js` features and use.

**If this is your first time using `reveal.js`, we encourage you to explore [the official demo](https://revealjs.com/demo/#/2) to see what sort of things you can do with it!**
We are creating and customizing slides with [`reveal-md`](https://github.com/webpro/reveal-md): a tool built with `reveal.js` to allow for [Markdown](https://commonmark.org/help/) only slides, with a few extra syntax items to make _your slides look and feel awesome_ with as little effort as possible on style and visual design.

##### Copy & Paste Slides

The [Copy and Paste Slide Templates](./copy-paste-slides/page.md) page and source for the embedded slideshow demonstrate use and code snippets to accommodate **many common slide archetypes**.
It should be possible to modify examples in your slides from these templates, including:

- Multi-column slides
- Embedded media
- Diagrams (mermaid, and more)

### Working on Pages with `mdBook`

Pages embed slides, and may include links to materials, references and other things **_when it's impractical to include it within speaker notes for slides_**.
Most pages are just "stub" files to embed the slides into.

To work on both the embedded slides and the book in tandem in **non-watching mode**:

```sh
makers s # Build the slides (clobbering those tracked by the book repo in `./slides`), embed in the book, view the updated book.

# ... Make changes to the book and/or the slides ...
# ... kill the server with `ctrl+c` ...

makers s # Build the slides (clobbering those tracked by the book repo in `./slides`), embed in the book, view the updated book.
```

> 😭 At this time, this is a non-watching server, you must manually open the page and _hard refresh_ pages served before to see them updated.
>
> You must _rerun_ this command to update on file changes!

### Lesson Template

Head over to the [Lesson Template](./template/page.md) page, and _carefully read through the source_ before you continue.
The entire directory is intended to be copied & pasted into the correct module to kickoff new lesson development:

```sh
# Copy this whole thing 👇😀
└── template
   ├── img
   │  └── REMOVE-ME-example-img.png
   ├── page.md
   └── slides.md
```

The `page.md` file should \_embed the `slides.html` page that isn't going to work until the build process creates it, but it will exist once that happens & render 😉.

### File Size Considerations

We strive to not overload this book with excessively large assets, to that end we ask that all contributors _before committing to this repo_ any assets:

- Review image file size & compress minimal possible looking OK-ish full screen, or use smaller alternatives.
  Example:
  ```sh
  # Compress with imagemagick
  convert <INPUT_FILE> -quality 20% <OUTPUT_FILE>
  ```
- Scale down all videos to minimal possible looking OK-ish full screen.
  Example:
  ```sh
  # What is the bitrate?
  ffmpeg -i <INPUT_FILE> 2> >(grep -i bitrate)
  # Reduce bitrate, iterate to find the *good enough* one for minimal size
  ffmpeg -i <INPUT_FILE> -b 400k <OUTPUT_FILE>
  ```

### Refactoring Considerations

<details>
<summary>🚧 This workflow is not _normally_ needed by most contributors. Click to view anyway 🚧</summary>

We opt out of the handy helper to create missing files if linked in `SUMMARY.md`, as this indicates something is likely amis in our translation of slides -> stub pages mapping correctly.

This is useful to turn back on when _radically_ updating the slides path structure and/or file names as changes must be _manually_ applied otherwise to link to the correct new location in `/slides/.../*-slides.html`

You can opt in by editing `book.toml`:

```diff,toml
[build]
- create-missing = false # do not create missing pages
+ create-missing = true # create missing pages
```

### Tips on the Embedded Slides

All modules are of the structure described in the [Content Organization](#organization) section.

All `slides.md` files are the source of the associated slide content for that the `page.md` files that embed them in the book itself.
The `page.md` files are typically just stubs, but do the option to add more details, instructions, etc.
They typically are identical to:

```md
# SOME TITLE HERE

<!-- markdown-link-check-disable -->

<iframe style="width: 90%; aspect-ratio: 1400/900; margin: 0 5%; border: none;" src="slides.html"></iframe>

<center>
<a target="_blank" href="../../contribute/how-to/page.md#-how-to-use-revealjs-slides"><i class="fa fa-pencil-square"></i> How to use the slides</a> -
<a target="_blank" href="slides.html"><i class="fa fa-share-square"></i> Full screen slides (new tab)</a>
</center>

<details>
<summary>Raw markdown of slides</summary>
{ {#include slides.md} }
<!-- 👆 REMOVE the spaces in curly brackets ( {{include things}} ) when in use -- mdBook gives a build error without mangling the syntax here in the example 😜 -->
</details>

<!-- markdown-link-check-enable -->
```

- `find . -name 'page.md' -exec bash -c 'cat ../tmp  >> "{}"' \;` to apply the page stuff to embed slides

</details>

### ⏰ Critical Considerations

- Always use proper MarkDown links!
  `<https://parity.io>` is required, raw links _will not be rendered_ in mdBook!
- Never use links that are likely ephemeral and will break.
  This example is in main, not some PR branch: <https://github.com/paritytech-stg/polkadot-sdk/blob/5174b9d/polkadot/roadmap/implementers-guide/src/node/backing/prospective-parachains.md>
  You _must_ be permalinks to a commit hash when using a github link, not `main` or other branch.
- Reuse images and have no duplication of any images, with close _enough_ ones considered to replace where possible.
  **_Relative_** paths are supported: `../../<other lesson>/img/<existing img>`

---

## Conventions and Helpers

This book, and all content within have style and typographic conventions that, where practical, have tooling to help everyone conform to them.

### Formatting Markdown, TOML, JSON

We enforce the use of a few formatters, the primary one being [Prettier](https://prettier.io/) that is included in the dev-dependencies for this repository.
In `package.json` used in the `cargo make` tooling we include easy access to run this:

```sh
# This will format all `content/*.md` files
makers f
```

If (and only if) formatting _breaks_ markdown from rendering correctly, you may use `<!-- prettier-ignore -->` preceding a block in markdown to skip formatting like this:

````markdown
<!-- prettier-ignore -->
```html
<pba-cols>
  <pba-col>
    
    ### What's up, yo?

</pba-col>
<pba-col>
  
  - Yo
- Yo
- Yo

</pba-col>
</pba-cols>
```
````

The above ` ```html ` block will not be formatted.

### Checking Links

To ensure all `*.md` contain no broken links within them, we have included a [simple link checker](https://github.com/tcort/markdown-link-check) you can run per module of content with:

```sh
# Link check a single file:
makers links-for <relative-link-to/the-top-working-dir/file.md>
# Regex glob match works too:
makers links-for ./content/contribute/**/*.md

# Link check all content files
makers l
```

The checker uses the `.github/workflows/mlc_config.json` configuration, primarily used to _globally_ ignore specific common URLs that throw errors, in error 😛.
_Notice that ignored links must be check from time to time manually!_
_Thus don't use this unless explicitly needed, rather use a know good URL if at all possible, perhaps from the <https://web.archive.org/>_
The same tool is also run by our CI on all files for all pushes to all branches.
See the `.github/workflows/link-check.yml` file in this repo for details.

> You can ignore the link check for a single line by post-fixing it with `<!-- markdown-link-check-disable-line -->`:
>
> ```markdown
> Some [private](nonono) or intentionally [broken link](FIXME). <!-- Remove the following disable once <something> happens --> <!-- markdown-link-check-disable-line -->`
> ```

## CI

On any merge with `main`, the CI is tasked with building the book and deploying a hosted version of it.

See `.github/workflows/` in this repository for more details.
Other tasks mostly stand alone from the `cargo make` tooling suggested in development workflows at this time, but does require the `yarn` tooling to properly build and test things.
The workflows consist of minimal checks for style, common errors, formatting, etc.
