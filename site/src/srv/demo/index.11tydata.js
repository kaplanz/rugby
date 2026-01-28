import { readFileSync } from "fs";
import { join } from "path";

export default {
  eleventyComputed: {
    demoAssets: () => {
      try {
        // Read the built index.html from the web app
        const htmlPath = join(process.cwd(), "../apps/web/dist/index.html");
        const html = readFileSync(htmlPath, "utf-8");

        // Extract JS asset path
        const jsMatch = html.match(/src="(\/assets\/index-[^"]+\.js)"/);
        const cssMatch = html.match(/href="(\/assets\/index-[^"]+\.css)"/);

        return {
          js: jsMatch ? jsMatch[1] : null,
          css: cssMatch ? cssMatch[1] : null,
        };
      } catch (err) {
        console.warn("Could not read demo assets:", err.message);
        return { js: null, css: null };
      }
    },
  },
};
