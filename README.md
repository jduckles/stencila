<div align="center">
	<a href="https://stenci.la/about">
		<img src="https://raw.githubusercontent.com/stencila/stencila/master/images/logo-name.png" alt="Stencila">
	</a>
	<br>
	<a href="#about">About</a>&nbsp;|&nbsp;<a href="#packages">Packages</a>&nbsp;|&nbsp;<a href="#discuss">Discuss</a>&nbsp;|&nbsp;<a href="#develop">Develop</a>
	<br>
</div>

### About

[Stencila](http://stenci.la) is a platform for creating, collaborating on, and sharing data driven content. Content that is **transparent** and **reproducible**, like [RMarkdown](https://github.com/rstudio/rmarkdown) and [Jupyter](http://jupyter.org/). Content that can be **versioned** and **composed** just like we do with open source software using tools like [CRAN](https://cran.r-project.org/web/packages/available_packages_by_name.html) and [NPM](https://www.npmjs.com/). And above all, content that is **accessible** to non-coders, like [Google Docs](https://en.wikipedia.org/wiki/Google_Docs,_Sheets_and_Slides) and [Microsoft Office](https://en.wikipedia.org/wiki/Microsoft_Office).

<img src="https://raw.githubusercontent.com/stencila/stencila/master/images/document-screenshot.png" alt="Stencila Document">

### Packages

Stencila is implemented as an ecosystem of software packages, each exposing the platform to different types of users and use cases. For example, if you just want a quick way to get started, check out the [desktop application](https://github.com/stencila/electron); but if you're a Python coder, the [`py`](https://github.com/stencila/py) package, will probably be more your thing. This repository doesn't contain any active code itself, it's just here as a central place for shared documentation and issues.

We're currently undertaking a major rewrite of the platform, moving to a more modular and decoupled architecture. It's still a work in progress. So if you're confused or what more info [talk to us](https://gitter.im/stencila/stencila). (Really, do it! :smile:)

A cross platform desktop application is provided by the `electron` package:

- [electron](https://github.com/stencila/electron) : Stencila on the desktop

Browser-based user interfaces are provided by the `web` package: 

- [web](https://github.com/stencila/web) : Stencila in the browser

Currently, four packages expose the platform in alternative host languages (packages for other languages, including Julia and Go, are planned)

- [node](https://github.com/stencila/node) : Stencila for Node.js
- [py](https://github.com/stencila/py) : Stencila for Python
- [r](https://github.com/stencila/r) : Stencila for R
- [cpp](https://github.com/stencila/cpp): Stencila for C++

These packages are in various stages of development. Together they act as an ecosystem of peers with differing capabilities. The `node` package is the current focus for implementation mainly because it can be easily integrated into the `electron` package and made available as a desktop application. But we aim to implement the same capabilities in the other language packages so that eventually can each be used standalone.

### Discuss

We love feedback. Chat with members of the community on [Gitter](https://gitter.im/stencila/stencila). For general suggestions you can create a [new issue](issues/new) or add to [existing issues](stencila/issues) in this repo. For package specific issues, please go to Issues page for the package repository (links below). 

### Develop

Want to help out with development? Great, there's a lot to do! To get started, [get in touch](https://gitter.im/stencila/stencila) or checkout the [platform-wide, cross-repository kanban board](https://github.com/orgs/stencila/projects/1).

               | Build status | Test coverage | Issues
-------------- |:------------:|:-------------:|:------:
[cpp](https://github.com/stencila/cpp) | [![Build status](https://travis-ci.org/stencila/cpp.svg?branch=master)](https://travis-ci.org/stencila/cpp) | [![Test coverage](https://codecov.io/gh/stencila/cpp/branch/master/graph/badge.svg)](https://codecov.io/gh/stencila/cpp) | [![Issues](http://img.shields.io/github/issues/stencila/cpp.svg)]( https://github.com/stencila/cpp/issues )
[electron](https://github.com/stencila/electron) | [![Build status](https://travis-ci.org/stencila/electron.svg?branch=master)](https://travis-ci.org/stencila/electron) | [![Test coverage](https://codecov.io/gh/stencila/electron/branch/master/graph/badge.svg)](https://codecov.io/gh/stencila/electron) | [![Issues](http://img.shields.io/github/issues/stencila/electron.svg)]( https://github.com/stencila/electron/issues )
[node](https://github.com/stencila/node) | [![Build status](https://travis-ci.org/stencila/node.svg?branch=master)](https://travis-ci.org/stencila/node) | [![Test coverage](https://codecov.io/gh/stencila/node/branch/master/graph/badge.svg)](https://codecov.io/gh/stencila/node) | [![Issues](http://img.shields.io/github/issues/stencila/node.svg)]( https://github.com/stencila/node/issues )
[py](https://github.com/stencila/py) | [![Build status](https://travis-ci.org/stencila/py.svg?branch=master)](https://travis-ci.org/stencila/py) | [![Test coverage](https://codecov.io/gh/stencila/py/branch/master/graph/badge.svg)](https://codecov.io/gh/stencila/py) | [![Issues](http://img.shields.io/github/issues/stencila/py.svg)]( https://github.com/stencila/py/issues )
[r](https://github.com/stencila/r) | [![Build status](https://travis-ci.org/stencila/r.svg?branch=master)](https://travis-ci.org/stencila/r) | [![Test coverage](https://codecov.io/gh/stencila/r/branch/master/graph/badge.svg)](https://codecov.io/gh/stencila/r) | [![Issues](http://img.shields.io/github/issues/stencila/r.svg)]( https://github.com/stencila/r/issues )
[web](https://github.com/stencila/web) | [![Build status](https://travis-ci.org/stencila/web.svg?branch=master)](https://travis-ci.org/stencila/web) | [![Test coverage](https://codecov.io/gh/stencila/web/branch/master/graph/badge.svg)](https://codecov.io/gh/stencila/web) | [![Issues](http://img.shields.io/github/issues/stencila/web.svg)]( https://github.com/stencila/web/issues )

### Museum

As with many software projects, Stencila has evolved a lot as we have learned what works, and what doesn't. Two older branches have been preserved in this repo for historical interest (and because they contain a significant amount of work and some useful code!):

#### [`triassic`](https://github.com/stencila/stencila/tree/triassic) branch c. 2012-2013. 305 commits

Initial explorations and experiments. This branch, introduces `Stencils` (what we now call `Documents`) as data-driven, programmable documents. Stencils are HTML documents containing directives such as `script` (for executing code) and `text` (for evaluating expressions and rendering them as text) encoded as element attributes e.g. `<span data-text="6*7">42</span>`. A mark up language called *Stem* (later renamed to *Cila*) was developed as an alternative to HTML for authoring. Stem attempts to take the combine the simplicity of [Markdown](https://daringfireball.net/projects/markdown/) with the flexibility of web templating languages like [Slim](http://slim-lang.com/).  In addition, this branch includes work on abstractions for data tables and queries on them (like in [dplyr](https://cran.rstudio.com/web/packages/dplyr/vignettes/introduction.html) and [blaze](http://blaze.readthedocs.io/en/latest/index.html)). The implementation strategy in this branch (and the next) was to develop mostly in C++ (making the most of libraries such as [pugixml](http://pugixml.org/) and [tidy-html5](https://github.com/htacg/tidy-html5)) and then expose this functionality using thin wrappers in host languages, initially R and Python. This branch also includes some initial work on browser-based user interfaces. 

#### [`jurassic`](https://github.com/stencila/stencila/tree/jurassic) branch c. 2014-2016. 2414 commits

Clean up and focus. This branch started as a fresh slate to remove the experiments, cleanup the implementation and focus on `Stencils` as the central part of the platform. This branch also marks the start (and probably the end) of [releases for this repository](https://github.com/stencila/stencila/releases). Releases 0.1 to 0.4 really just set things up. Execution contexts for rendering stencils in R and Python were introduced in [0.5](https://github.com/stencila/stencila/releases/tag/0.5) and *Stem* was re-released as *Cila* in [0.6](https://github.com/stencila/stencila/releases/tag/0.6). Releases 0.7 to 0.18 were mostly incremental improvements (although together they add up to big improvements in reliability and usability!). Release [0.19](https://github.com/stencila/stencila/releases/tag/0.19) was an important because it was the first to use [Substance](http://substance.io) and was thus a big step up for our browser-based [user interfaces](https://twitter.com/_substance/status/661440688211501056). We went on to leverage Substance and the execution contexts we had already developed to introduce [`Sheets`](https://stenci.la/stencila/blog/introducing-sheets/) in [0.21](https://github.com/stencila/stencila/releases/tag/0.21). Subsequent releases focused on interoperability including introducing a Node.js package in [0.23](https://github.com/stencila/stencila/releases/tag/0.23) and RMarkdown support for stencils in [0.24](https://github.com/stencila/stencila/releases/tag/0.24).

#### [`master`](https://github.com/stencila/stencila/tree/master) branch c. 2016-

The implementation strategy we had been using (core in C++ with wrappers for various languages) had advantages (a single fast implementation deployed to various languages) but a few significant problems. Prime amongst these problems was that it was difficult for others to contribute code, particularly those who specialized in a single higher level language e.g R. To address these issues, our current strategy is to use a decoupled, modular architecture with various packages linked via network communications rather than an underlying C++ core. This also opens up some exciting possibilities for distributed and polyglot computing.
