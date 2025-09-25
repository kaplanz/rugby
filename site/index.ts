import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";

import pino from "pino";

const log = pino({
  level: "trace",
  transport: {
    target: "pino-pretty",
  },
});

import { select } from "hast-util-select";
import { unified } from "unified";

import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeExternalLinks from "rehype-external-links";
import rehypeParse from "rehype-parse";
import rehypeStringify from "rehype-stringify";
import rehypeSlug from "rehype-slug";

import remarkCallouts from "remark-callouts";
import remarkGfm from "remark-gfm";
import remarkInlineFootnotes from "remark-inline-footnotes";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";

const ROOT = path.join(fileURLToPath(import.meta.url), "..");

const SRC = path.join(ROOT, "src");
const WWW = path.join(ROOT, "www");
const TMP = path.join(ROOT, "tmp/base.html");
const OUT = path.join(ROOT, "dist");

async function main() {
  // Define markdown parser
  const parser = async (src: string) => await unified()
    .use(remarkParse, { fragment: true })
    .use(remarkGfm)
    .use(remarkCallouts)
    .use(remarkInlineFootnotes)
    .use(remarkRehype, { allowDangerousHtml: true })
    .use(rehypeExternalLinks, {
      target: "_blank",
    })
    .use(rehypeSlug)
    .use(rehypeAutolinkHeadings, {
      behavior: "wrap",
      properties: {
        className: "anchor",
        rel: "nofollow",
      },
    })
    .use(rehypeStringify, { allowDangerousHtml: true })
    .process(src);

  // Find markdown files
  const src = [
    ...(await find(SRC))
      .filter(f => path.extname(f) === ".md")
      .map(f => new Page(f, {
        prefix: {
          del: "src",
        },
      })),
    ...(await find(path.join(ROOT, "..")))
      .filter(f => path.basename(f) === "README.md")
      .filter(f => path.dirname(f) !== ".")
      .map(f => new Page(f, {
        rename: "index",
        prefix: {
          del: "..",
        },
      })),
  ];

  // Parse template file
  const tmp = unified()
    .use(rehypeParse)
    .parse(await fs.readFile(TMP));

  log.info("compiling markdown");
  for (const page of src) {
    // Read any parse markdown
    const text = await fs.readFile(page.src);
    const body = await parser(text.toString());

    // Clone template for page
    const base = structuredClone(tmp);
    const node = select("main", base);
    if (!node)
      throw new Error("template is missing node");
    // Insert content into node
    node.children = [{ type: "raw", value: body.toString() }];

    // Produce output HTML
    const html = unified()
      .use(rehypeStringify, { allowDangerousHtml: true })
      .stringify(base);

    // Make output directory
    const out = path.relative(
      process.cwd(),
      path.join(OUT, page.uri)
    );
    await fs.mkdir(path.dirname(out), { recursive: true });
    // Write output file
    await fs.writeFile(out, html);
    log.debug(`wrote ${out}`);
  }

  // Copy static files
  log.info("copying static files");
  await fs.cp(WWW, OUT, {
    recursive: true,
  });
}

async function find(root: string) {
  return await fs.exists(root)
    ? await Bun.spawn(
      [
        "git",
        "ls-files",
        "-z",
        "--others",
        "--cached",
        "--exclude-standard",
        "--no-empty-directory",
      ], { cwd: root }
    ).stdout.text().then(
      out => out.split("\0").map(f => path.relative(
        process.cwd(),
        path.join(root, f),
      ))
    )
    : []
}

class Page {
  src: string;
  uri: string;

  constructor(file: string, opts: Transform = {}) {
    // Source filepath
    this.src = file;

    // Strip matched prefix
    if (opts.prefix?.del && file.startsWith(opts.prefix.del)) {
      file = file.slice(opts.prefix.del.length);
      // Fix leading slashes
      if (file.startsWith("/"))
        file = file.slice(1);
    }
    // Prepend new prefix
    if (opts.prefix?.add) {
      file = path.join(opts.prefix.add, file);
    }

    // Resolve URI name
    file = path.join(
      path.dirname(file),
      opts.rename ?? path.basename(file, path.extname(file))
    );

    // Resolve URI path
    file = path.format({
      ...path.parse(file),
      ...{ base: undefined, ext: ".html" },
    });

    // Resource identifier
    this.uri = file
  }
}

type Transform = {
  rename?: string;
  prefix?: {
    del?: string;
    add?: string;
  };
};

await main();
