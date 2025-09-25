import * as Unist from "unist";
import { u } from "unist-builder";
import { visit } from "unist-util-visit";

// Remark plugin for callouts.
//
// Transform blockquotes with callout directives (e.g., [!NOTE]) into styled
// HTML elements.
export default function remarkCallouts() {
  return (tree: Unist.Node) => {
    // Traverse the Markdown AST and find blockquote nodes
    visit(tree, "blockquote", (node: any, index: number, parent: Unist.Parent) => {
      // Check for the callout pattern in the first line of the blockquote
      const cell = node.children?.[0]?.children?.[0];
      const regx = cell?.value?.trim().match(/^\[!(\w+)\]/s);
      if (!regx) return;

      // Extract callout type (e.g., note, tip, warning)
      cell.value = cell.value.slice(regx[0].length);
      const type = regx[1];

      // Replace blockquote with a div representing the callout
      parent.children.splice(
        index,
        1,
        u("blockquote", {
          data: {
            hProperties: {
              className: ["callout", type.toLowerCase()],
            },
          },
        }, node.children),
      );
    });
  };
}
