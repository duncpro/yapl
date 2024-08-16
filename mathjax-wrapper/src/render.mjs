import * as mathjax from "mathjax";

// MathJax provides a variety of conversions, however yapl is concerned only with
// the TeX to SVG conversion, so we load just that.
const MathJax = await mathjax.init({ loader: { load: ['input/tex', 'output/svg'] } });

export function renderSVG(tex, preserveAspectRatio) {
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

export function renderCSS() { 
  const stylesheet = MathJax.svgStylesheet();
  MathJax.startup.output.clearCache();
  return stylesheet;
}
