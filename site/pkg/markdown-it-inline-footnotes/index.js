import crypto from "crypto";
import markdownItFootnote from "markdown-it-footnote";

// markdown-it plugin for inline footnotes with checkboxes
// Converts footnote references into interactive inline footnotes
export default function markdownItInlineFootnotes(md) {
  // First, load the standard footnote plugin
  md.use(markdownItFootnote);

  // Store tokens in env for on-demand lookup
  const originalRender = md.render.bind(md);
  md.render = function(src, env) {
    const tokens = md.parse(src, env || {});
    if (!env) env = {};
    env._allTokens = tokens;
    return md.renderer.render(tokens, md.options, env);
  };

  // Override the footnote reference renderer
  md.renderer.rules.footnote_ref = function(tokens, idx, options, env, self) {
    const token = tokens[idx];
    const id = token.meta.id;
    const ident = crypto.randomBytes(4).toString("hex");

    // Find footnote definition in token stream and render on-demand
    let content = '';
    if (env && env._allTokens) {
      for (let i = 0; i < env._allTokens.length; i++) {
        if (env._allTokens[i].type === 'footnote_open' &&
            env._allTokens[i].meta.id === id) {
          // Find the first paragraph's inline content
          for (let j = i + 1; j < env._allTokens.length; j++) {
            if (env._allTokens[j].type === 'footnote_close') break;
            if (env._allTokens[j].type === 'paragraph_open' &&
                j + 1 < env._allTokens.length &&
                env._allTokens[j + 1].type === 'inline') {
              const inlineToken = env._allTokens[j + 1];
              // Render inline content - nested footnotes will recursively call this renderer
              content = md.renderer.renderInline(inlineToken.children || [], options, env);
              break;
            }
          }
          break;
        }
      }
    }

    return `<span class="footnote">` +
           `<label for="footnote:${ident}"></label>` +
           `<input id="footnote:${ident}" type="checkbox">` +
           `<span><span>${content}</span></span>` +
           `</span>`;
  };

  // Hide the footnote block at the bottom
  md.renderer.rules.footnote_block_open = function() {
    return '<section class="footnotes" style="display: none;">\n';
  };

  return md;
}
