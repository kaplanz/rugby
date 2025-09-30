import path from "path";

import { InputPathToUrlTransformPlugin } from "@11ty/eleventy";

import { markdown } from "./src/markdown.js";

export default async function(cfg) {
  // Passthrough copy assets
  cfg.addPassthroughCopy({
    "./www": "/site/www",
  });

  // Customize permalinks
  cfg.addGlobalData("permalink", () => {
    return (data) => {
      // Parse URL
      const url = {
        ...path.parse(data.page.filePathStem),
        base: undefined,
        ext: data.page.outputFileExtension,
      };
      // Render READMEs as index files
      if (url.name === "README") {
        url.name = "index";
      }
      // Convert uppercase file names
      if (url.name == url.name.toUpperCase()) {
        url.name = url.name.toLowerCase();
      }
      // Remove trailing slashes
      return path.format(url);
    }
  });

  // Remove URL extensions
  cfg.addUrlTransform((page) => {
    const url = path.parse(page.url);
    if (url.ext)
      return page.url.slice(0, -1 * url.ext.length);
  });

  // Transform paths to URLs
  cfg.addPlugin(InputPathToUrlTransformPlugin);

  // Provide default layout
  cfg.addGlobalData("layout", "main");

  // Compute page breadcrumbs
  cfg.addGlobalData("eleventyComputed", {
    crumb: (data) => (data.page.url.match(/(^\/)|[^\/]+\/?/g) || []).reduce(
      (root, page) => [...root, {
        url: path.join(root.slice(-1)[0]?.url ?? "/", page),
        txt: page,
      }],
      [],
    ),
  });

  // Customize Markdown rendering
  cfg.setLibrary("md", { render: markdown });

  // Return static settings
  return {
    dir: {
      input: "..",
      includes: "site/include",
      data: "site/data",
      output: "dist"
    }
  };
};
