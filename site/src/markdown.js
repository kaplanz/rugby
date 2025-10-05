import { unified } from "unified";

import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeExternalLinks from "rehype-external-links";
import rehypeStringify from "rehype-stringify";
import rehypeSlug from "rehype-slug";

import remarkCallouts from "remark-callouts";
import remarkGfm from "remark-gfm";
import remarkInlineFootnotes from "remark-inline-footnotes";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";

export const markdown = (src) => unified()
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
  .processSync(src)
  .toString();
