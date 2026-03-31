import path from "path";

// 11ty
import { HtmlBasePlugin } from "@11ty/eleventy";

// markdown-it plugins
import markdownItDeflist from "markdown-it-deflist";
import markdownItAnchor from "markdown-it-anchor";
import markdownItGithubAlerts from "markdown-it-github-alerts";
import markdownItInlineFootnotes from "markdown-it-inline-footnotes";

// transforms
import { transform } from "lightningcss";
import htmlnano from "htmlnano";

// metadata
const site = {
  title: "Rugby | Game Boy Emulator",
  about: "Cycle-accurate Game Boy emulation",
  author: {
    name:  "Zakhary Kaplan",
    email: "me@zakhary.dev"
  },
  url: "https://rugby.zakhary.dev",
  lang: "en-CA",
};

// Shared function to transform .md file names to web-accessible names
function permalink(name) {
  if (name.toUpperCase() === "README") {
    return "index";
  }
  if (name === name.toUpperCase()) {
    return name.toLowerCase();
  }
  return name;
}

// eleventy
export default async function(cfg) {
  // Site metadata
  cfg.addGlobalData("site", site);
  // Build info
  cfg.addGlobalData("build", {
    date: new Date(),
  });
  // Page layout
  cfg.addGlobalData("layout", "main")

  // Remove trailing slashes
  //
  // See: https://www.11ty.dev/docs/permalinks/#remove-trailing-slashes
  //
  // Set global permalinks to resource.html style
  cfg.addGlobalData("permalink", () => {
    return data => {
      // Parse URL
      const url = {
        ...path.parse(data.page.filePathStem),
        base: undefined,
        ext: data.page.outputFileExtension,
      };
      // Transform .md file names
      url.name = permalink(url.name);
      // Remove trailing slashes
      return path.format(url);
    };
  });
  // Remove .html from `page.url`
  cfg.addUrlTransform(page => {
    if (page.url.endsWith(".html")) {
      return page.url.slice(0, -1 * ".html".length);
    }
  });

  // Transform paths
  cfg.addTransform("path-transform", (content, outputPath) => {
    if (typeof outputPath !== "string" || !outputPath.endsWith(".html"))
      return content;
    // Transform GitHub-style image paths to web-accessible paths
    return content.replace(/src="\.\/site\/src\/www\//g, 'src="/');
  });

  // Minify HTML
  cfg.addTransform("html-minify",async (content, outputPath) => {
    if (typeof outputPath !== "string" || !outputPath.endsWith(".html"))
      return content;
    const result = await htmlnano.process(content, {
      minifyCss: false,
      minifyJs:  false,
      minifySvg: false,
      collapseWhitespace: "aggressive"
    });
    return result.html;
  });

  // Per-page bundles
  //
  // See: https://github.com/11ty/eleventy-plugin-bundle
  //
  // Bundle <style> content and adds a {% css %} paired shortcode
  cfg.addBundle("css", {
    // Optional subfolder (relative to output directory) files will write to
    toFileDirectory: "assets/css",
    // Modify bundle content
    transforms: [
      function(content) {
        // type contains the bundle name.
        let { page } = this;
        let { code } = transform({
          filename: page.inputPath,
          code: Buffer.from(content),
          minify: true,
          sourceMap: true,
        });
        return code;
      }
    ],
    // Add all <style> content to `css` bundle
    //
    // (use <style eleventy:ignore> to opt-out)
    //
    // Supported selectors: https://www.npmjs.com/package/posthtml-match-helper
    bundleHtmlContentFromSelector: "style",
  });

  // Copy static assets
  cfg.addPassthroughCopy({ "src/www": "/" });
  // Copy web app demo
  cfg.addPassthroughCopy({ "node_modules/rugby-web/dist": "/demo" });

  // HTML base plugin
  cfg.addPlugin(HtmlBasePlugin);

  // Configure markdown-it library with plugins
  let markdown;
  cfg.amendLibrary("md", md => {
    // Store for external use
    markdown = md;

    // Enable GFM features
    md.configure({
      options: {
        linkify: true,
        typographer: true,
      },
    });

    // Enable plugins
    md.use(markdownItDeflist)
      .use(markdownItGithubAlerts)
      .use(markdownItInlineFootnotes)
      .use(markdownItAnchor, {
        permalink: markdownItAnchor.permalink.headerLink({
          safariReaderFix: true,
          class: "anchor",
        })
      });

    // External links
    //
    // Add `target="_blank"` to external links.
    md.renderer.rules.link_open = function(tokens, idx, options, _env, self) {
      const token = tokens[idx];
      const index = token.attrIndex("href");
      if (index >= 0) {
        let href = token.attrs[index][1];

        // Transform markdown links
        if (href && path.extname(href.split('#')[0]) === '.md') {
          const [pathPart, ...anchorParts] = href.split('#');
          const anchor = anchorParts.length > 0 ? '#' + anchorParts.join('#') : '';
          const { dir, name } = path.parse(pathPart);
          href = path.join(dir, name);

          // Handle ./path or /path
          if (href.startsWith("./")) {
            href = href.substring(1); // Remove leading .
          }

          // Transform the filename using shared logic
          const parts = href.split("/");
          const filename = parts[parts.length - 1];
          parts[parts.length - 1] = permalink(filename);
          href = parts.join("/");

          // Ensure leading slash and add anchor
          if (!href.startsWith("/")) {
            href = "/" + href;
          }
          href += anchor;

          token.attrs[index][1] = href;
        }

        // Check if it's an external link
        if (URL.canParse(href)) {
          token.attrSet("target", "_blank");
        }
      }
      return self.renderToken(tokens, idx, options);
    };

    // Alert renderer
    //
    // Use blockquote tags instead of div tags.
    md.renderer.rules.alert_open = function(tokens, idx) {
      const { type } = tokens[idx].meta;
      return `<blockquote class="callout ${type}">`;
    };
    md.renderer.rules.alert_close = function() {
      return "</blockquote>\n";
    };

    // Image transformer
    //
    // Transform GitHub-style paths to web-accessible paths.
    const originalImageRenderer = md.renderer.rules.image || function(tokens, idx, options, env, self) {
      return self.renderToken(tokens, idx, options);
    };
    md.renderer.rules.image = function(tokens, idx, options, env, self) {
      const token = tokens[idx];
      const srcIndex = token.attrIndex("src");
      if (srcIndex >= 0) {
        let src = token.attrs[srcIndex][1];
        // Transform ./site/src/www paths to /
        if (src?.includes("/site/src/www/")) {
          src = src.replace(/.*\/site\/src\/www\//, "/");
          token.attrs[srcIndex][1] = src;
        }
      }
      return originalImageRenderer(tokens, idx, options, env, self);
    };

    return md;
  });

  // Add markdown filter
  cfg.addFilter("markdown", content => markdown.render(content));
  // Inspect filter
  cfg.addFilter("inspect", data => {
    console.log(data);
    return data;
  });
  // Split filter
  cfg.addFilter("split", (str, separator) => {
    return str ? str.split(separator) : [];
  });

  // Layout aliases for cleaner front matter
  cfg.addLayoutAlias("main", "main.njk");

  // Watch for changes
  cfg.addWatchTarget("./pkg/");
}

export const config = {
  // Control which files Eleventy will process
  // e.g.: *.md, *.njk, *.html, *.liquid
  templateFormats: [
    "md",
    "njk",
    "html",
    "liquid",
    "11ty.js",
  ],

  // Pre-process *.md files with: (default: `liquid`)
  markdownTemplateEngine: "njk",

  // Pre-process *.html files with: (default: `liquid`)
  htmlTemplateEngine: "njk",

  // These are all optional:
  dir: {
    input: "..",              // default: "."
    includes: "site/src/lib", // default: "_includes" (`input` relative)
    data: "site/src/etc",     // default: "_data" (`input` relative)
    output: "dist",
    layouts: "site/src/lib/layout",
  },

  // -----------------------------------------------------------------
  // Optional items:
  // -----------------------------------------------------------------

  // If your site deploys to a subdirectory, change `pathPrefix`.
  //
  // Read more:
  // https://www.11ty.dev/docs/config/#deploy-to-a-subdirectory-with-a-path-prefix

  // When paired with the HTML <base> plugin
  // it will transform any absolute URLs in your HTML to include this
  // folder name and does **not** affect where things go in the output folder.
  //
  // Read more: https://www.11ty.dev/docs/plugins/html-base/

  // pathPrefix: "/",
}
