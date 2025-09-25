import * as Mdast from "mdast";
import * as Unist from "unist";
import { u } from "unist-builder";
import { visit } from "unist-util-visit";

export default function remarkInlineFootnotes() {
  return (tree: Unist.Node) => {
    visit(tree, "footnoteDefinition", (def: Mdast.Association & Mdast.Parent) => {
      // Replace footnote references with inline footnotes
      visit(tree, "footnoteReference", (ref: Mdast.Association, index, parent: Unist.Parent) => {
        // Only modify reference for this definition
        if (ref.identifier !== def.identifier)
          return;

        // Insert footnote reference
        parent.children.splice(
          index,
          1,
          u("span", {
              data: {
                hProperties: {
                  className: "footnote",
                },
              },
            }, [
              u("text", {
                  data: {
                    hProperties: {
                      for: `sn-toggle-${def.identifier}`,
                    },
                  },
                }, "",
              ),
              u("text", {
                  data: {
                    hName: "input",
                    hProperties: {
                      id: `sn-toggle-${def.identifier}`,
                      type: "checkbox",
                    },
                  },
                }, "",
              ),
              u("span", {}, [
                  u("span", {}, def.children?.[0]?.children),
                ],
              ),
            ],
          ),
        );
      });
    });
  };
}
