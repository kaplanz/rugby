import { createElement } from "lucide";
import type { IconNode } from "lucide";

/**
 * Renders a Lucide icon as an SVG element.
 */
export const icon = (nodes: IconNode): SVGElement =>
  createElement(nodes, { "aria-hidden": "true", height: "1em", width: "1em" });
