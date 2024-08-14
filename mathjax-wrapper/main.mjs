import * as mathjax from "mathjax";
import { read } from "./util.mjs";

const MathJax = await mathjax.init({ loader: { load: ['input/tex', 'output/svg'] } });

const tex = await read(process.stdin);

// In spite of  asking for SVG specifically, MathJax still wraps the SVG in HTML.
const html_container = MathJax.tex2svg(tex, {display: false});

// This is the SVG itself.
const svg = html_container.children[0];
delete svg.attributes.height;
delete svg.attributes.width;

const args = process.argv.slice(2);
if (args.length == 1) { svg.attributes.preserveAspectRatio = args[0]; }

console.log(MathJax.startup.adaptor.outerHTML(svg));
