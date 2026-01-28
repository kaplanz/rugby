# site

Homepage website for Rugby. Built with [Eleventy][11ty].

> [!NOTE]
>
> The site renders all markdown files from the parent directory (the repository
> root), converting `README.md` files to index pages.

## Usage

Requires [Bun][bun] to be installed.

```sh
bun install        # install dependencies
bun run build      # build production site to ./dist
bun run dev        # start local dev server at localhost:8080
```

## Organization

```
./
├── eleventy.config.js  # 11ty configuration
├── package.json        # project manifest
├── README.md           # this document
├── ...
├── pkg/                # package plugins
└── src/                # source files
   ├── etc/             # configuration
   ├── lib/             # templates
   │  ├── layout/       # page layouts
   │  └── widget/       # components
   └── www/             # static content
       ├── ...
       └── assets/      # static assets
           ├── css/     # stylesheets
           └── img/     # image files
```

## License

For information regarding licensure, please see the project's [README][license].

<!--
  Reference-style links
-->

[11ty]:    https://www.11ty.dev
[bun]:     https://bun.sh
[license]: /README.md#license
