// @ts-ignore
import * as mathjax from "mathjax";

// MathJax provides a variety of conversions, however yapl is concerned only with
// the TeX to SVG conversion, so we load just that.
const MathJax = await mathjax.init({ loader: { load: ['input/tex', 'output/svg'] } });

export function renderSVG(tex: string, preserveAspectRatio: string): string {
  // MathJax wraps the SVG in and HTML container element.
  // Since yapl generates pure SVGs, this container element must be removed.
  const html_container = MathJax.tex2svg(tex, {display: false});
  const svg = html_container.children[0];
  
  // Remove the width and height attributes of the <svg> tag so that the SVG scales to fill.
  // Then, on the Rust side, this SVG is wrapped in another SVG tag which will determine its size.
  delete svg.attributes.height;
  delete svg.attributes.width;
  
  if (preserveAspectRatio.length > 0) { 
    svg.attributes.preserveAspectRatio = preserveAspectRatio;
  }
  
  return MathJax.startup.adaptor.outerHTML(svg);
}

export function dumpCSS() { 
  const stylesheet = MathJax.svgStylesheet();

  // Curerntly not necessary, nor defined, since MathJax does not support stylesheet pruning
  // for the SVG target. Hopefully in the future they will support pruning for SVG, just
  // like they do currently for HTML. If/when that happens, this can be uncommented.
  // MathJax.startup.output.clearCache();
  
  return MathJax.startup.adaptor.innerHTML(stylesheet);
}
