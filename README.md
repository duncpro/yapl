# yapl
Experimental SVG plotting library with focus on mathematical exposition.

![sin(1/x)](readme-assets/sin1overx.png)

The end-goal of this library is to be a less-featureful replacement for 
[JSXGraph](https://jsxgraph.uni-bayreuth.de/wp/index.html).

However, that goal is a long way from being realized. At the moment this library
provides little more than a linear interpolation algorithm.

## Features

### Axis 
![sinx](readme-assets/sinx.png)

```rust
use std::f64::consts::PI;
use yapl::elements::{Function, CoordinatePlane, Axis, TickLabelKind, SymbolicTickLabel, TickLabel};
use yapl::math::{NonDecreasing, ClosedInterval};
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;
use yapl::style::Stylesheet;

fn main() -> std::io::Result<()> {
    let mut cplane = CoordinatePlane::new_elementary();

    let mut x_axis = Axis::new_default(0.0, PI, 0.0);
    x_axis.tick_label = Some(TickLabel::new_default(TickLabelKind::Symbolic(SymbolicTickLabel {
        offset_symbol_tex: None,
        stride_symbol_tex: "\\pi",
    })));
    cplane.horizontal_axis = Some(x_axis);
    
    cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-2.0 * PI - 1.0, 2.0 * PI + 1.0));
    cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.5, 1.5));

    cplane.fns.push(Function::new_default(|x| x.sin()));
    
    let stylesheet = Stylesheet::new_default();
    let mut tex_renderer = MathJaxProcessTeXRenderer::new();
    codegen(&mut std::io::stdout(), &cplane, stylesheet, &mut tex_renderer)
}
```

## To Do 
- Draw grid.
- Draw axis labels (not axis tick labels, we already draw those).
- Draw labeled points of interest.
- Draw labeled line segments.
- Draw basic shapes.
- Compile to WASM and expose an API to Javascript.
- Create interactive demonstration by implementing desmos-like webapp.

## Philosophy
### Styling

All styles are applied using CSS and not a Rust-based styling API. For instance there 
is no `Axis#color` field or the like. CSS is quite ergonmic already, and I see no need to
implement a clunky Rust API on top of it. 

This library provides a minimal default stylesheet. This is so that ledigble diagrams are generated
even if the user provides no explicit styling. The stylesheet is small enough to fit in only 
a few lines so that it is very clear what is going on and how the behavior can be changed. 

This library exposes a simple flag-based API for opting out of default style rules
and default style  classes on per-element and global basis. This provide a way for the user
to omit a default styling rule entirely if they wish to override it with a rule in their own 
custom stylesheet. This keeps redundant default styles out of the finished SVG.

A custom stylesheet can be applied in addition to the default stylesheet using 
`yapl::style::Stylesheet#custom`. The default stylesheet can be thinned or purged entirely
using `yapl::style::Stylesheet#defaults`. 
