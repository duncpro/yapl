// # SVG Code Generation

use crate::elements::{CoordinatePlane, Extent, TickLabel};
use crate::elements::{Function, FunctionKind};
use crate::math::{self, Vec2D, BoundingRect, ClosedInterval, NonDecreasing};
use crate::plotfn::{self, PlotFnParams};
use crate::misc::{SegVec, SegVecRoot};
use crate::typesetting::TeXRenderer;

pub fn codegen_cplane<W>(out: &mut W, cplane: &CoordinatePlane, 
    gstyle: &impl GlobalStyles, tex_renderer: &impl TeXRenderer)
-> std::io::Result<()>
where W: std::io::Write
{ 
    if cplane.extent.area() == 0.0 { return Ok(()); }

    let bound = normalize_coordinate(&cplane.extent, cplane.extent.brect.top_right());

    write!(out, "<svg")?;
    write!(out, " viewBox=\"0 0 {} {}\"", bound.x, bound.y)?;
    write!(out, " xmlns=\"http://www.w3.org/2000/svg\"")?;
    write!(out, " preserveAspectRatio=\"xMinYMin meet\"")?;
    write!(out, " ")?;
    gstyle.svg_root_attrs(out)?;
    write!(out, ">")?;    
    {
        let mut buf: SegVecRoot<plotfn::Node> = SegVecRoot::default();
        for function in &cplane.fns {
            codegen_fnplot(out, &cplane.extent, function, buf.extend(), gstyle)?;
        }
    }
    codegen_horizontal_axis(out, cplane, gstyle)?;
    codegen_vertical_axis(out, cplane, gstyle)?;
    codegen_horizontal_axis_ticks(out, cplane, gstyle)?;
    codegen_vertical_axis_ticks(out, cplane, gstyle)?;
    codegen_horizontal_axis_tick_labels(out, cplane, gstyle, tex_renderer)?;
    codegen_vertical_axis_tick_labels(out, cplane, gstyle, tex_renderer)?;
    write!(out, "</svg>")?;
    return Ok(())
}

pub trait GlobalStyles {
    fn function_stroke_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> { Ok(()) }
    fn axis_stroke_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> { Ok(()) }
    fn horizontal_axis_tick_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> { Ok(()) }
    fn vertical_axis_tick_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> { Ok(()) }
    fn both_axis_tick_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> { Ok(()) }
    fn svg_root_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> { Ok(()) }
    fn font_height(&self) -> f64;
}

pub struct DefaultGlobalStyles {
    pub function_stroke_width: f64,
    pub axis_stroke_width: f64,
    pub axis_tick_stroke_width: f64,
    pub font_height: f64
}

impl DefaultGlobalStyles {
    pub const DEFAULT_FUNCTION_STROKE_WIDTH: f64 = 1.0 / 400.0;
    pub const DEFAULT_AXIS_STROKE_WIDTH: f64 = 1.0 / 1000.0;
    pub const DEFAULT_AXIS_TICK_STROKE_WIDTH: f64 = 1.0 / 1000.0;
    pub const DEFAULT_FONT_HEIGHT: f64 = 2.0 / 100.0;
    
    pub fn new() -> Self {
        DefaultGlobalStyles { 
            function_stroke_width: Self::DEFAULT_FUNCTION_STROKE_WIDTH,
            axis_stroke_width: Self::DEFAULT_AXIS_STROKE_WIDTH,
            axis_tick_stroke_width: Self::DEFAULT_AXIS_TICK_STROKE_WIDTH,
            font_height: Self::DEFAULT_FONT_HEIGHT,
        }
    }
}

impl GlobalStyles for DefaultGlobalStyles {
    fn function_stroke_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(dest, "stroke-width=\"{}\"", self.function_stroke_width)
    }

    fn axis_stroke_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(dest, "stroke-width=\"{}\"", self.axis_stroke_width)
    }

    fn horizontal_axis_tick_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(dest, "stroke-width=\"{}\"", self.axis_tick_stroke_width)
    }
    
    fn vertical_axis_tick_attrs(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(dest, "stroke-width=\"{}\"", self.axis_tick_stroke_width)
    }

    fn font_height(&self) -> f64 { self.font_height }
}

fn codegen_fnplot<W>(out: &mut W, extent: &Extent, function: &Function, 
    mut buf: SegVec<plotfn::Node>, gstyles: &impl GlobalStyles)
-> std::io::Result<()>
where W: std::io::Write
{    
    let (domain, codomain) = match function.kind {
        FunctionKind::OfX => (extent.brect.x, extent.brect.y),
        FunctionKind::OfY => (extent.brect.y, extent.brect.x),
    };

    let error_tolerance = codomain.len() / function.error_tolerance_factor;
    let zero_tolerance = domain.len() / function.zero_tolerance_factor;
            
    plotfn::plotfn(&function.eval, &mut buf, PlotFnParams { domain, codomain, 
        min_depth: function.min_depth, error_tolerance, zero_tolerance }); 
       
    write!(out, "<path")?;
    write!(out, " stroke-linejoin=\"round\"")?;
    write!(out, " stroke-linecap=\"round\"")?;
    write!(out, " fill=\"none\"")?;
    write!(out, " stroke=\"black\"")?;
    write!(out, " ")?;
    gstyles.function_stroke_attrs(out)?;
    write!(out, " d=\"")?;

    let mut broken = true;
    for node in buf.as_slice().iter() {
        match node {
            plotfn::Node::Break => broken = true,
            plotfn::Node::Anchor(anchor) => {
                match broken {
                    true => write!(out, "M")?,
                    false => write!(out, "L")?,
                }
                broken = false;

                let x = match function.kind {
                    FunctionKind::OfX => anchor.input,
                    FunctionKind::OfY => (function.eval)(anchor.input),
                };

                let y = match function.kind {
                    FunctionKind::OfX => (function.eval)(anchor.input),
                    FunctionKind::OfY => anchor.input,
                };

                let coord = normalize_coordinate(extent, Vec2D { x, y });
                write!(out, " {} {} ", coord.x, coord.y)?;
            },
        }
    }
    
    write!(out, "\"/>")?;
    return Ok(())
}

fn codegen_vertical_axis<W>(out: &mut W, cplane: &CoordinatePlane, gstyle: &impl GlobalStyles) 
-> std::io::Result<()> 
where W: std::io::Write
{
    let Some(axis) = &cplane.vertical_axis else { return Ok(()); };
    
    let start = normalize_coordinate(&cplane.extent, Vec2D { 
        x: axis.pos,
        y: cplane.extent.brect.y.begin()
    });
    let stop = normalize_coordinate(&cplane.extent, Vec2D { 
        x: axis.pos,
        y: cplane.extent.brect.y.end() 
    });

    write_line_prefix(out, start, stop)?;
    write!(out, " stroke=\"black\"")?;
    write!(out, " ")?;
    gstyle.axis_stroke_attrs(out)?;
    write!(out, "/>")?;
    return Ok(());
}

fn codegen_horizontal_axis<W>(out: &mut W, cplane: &CoordinatePlane, style: &impl GlobalStyles) 
-> std::io::Result<()> 
where W: std::io::Write
{
   let Some(axis) = &cplane.horizontal_axis else { return Ok(()); };
    
    let start = normalize_coordinate(&cplane.extent, Vec2D {
        x: cplane.extent.brect.x.begin(),
        y: axis.pos
    });
    let stop = normalize_coordinate(&cplane.extent, Vec2D {
        x: cplane.extent.brect.x.end(),
        y: axis.pos
    });

    write_line_prefix(out, start, stop)?;
    write!(out, " stroke=\"black\"")?;
    write!(out, " ")?;
    style.axis_stroke_attrs(out)?;
    write!(out, "/>")?;
    return Ok(());
}

fn codegen_horizontal_axis_ticks<W>(out: &mut W, cplane: &CoordinatePlane, 
    gstyle: &impl GlobalStyles) -> std::io::Result<()> 
where W: std::io::Write
{
    let Some(axis) = &cplane.horizontal_axis else { return Ok(()); };
    if axis.stride == 0.0 { return Ok(()); }
    // to find out how many ticks have elapsed, subtract the position of the first tick
    // from the begin point of the visible interval and divide that quantity by the number
    // by the distance between consecutive ticks.
    let n = ((cplane.extent.brect.x.begin() - axis.offset) / axis.stride).ceil();
    let mut k = axis.offset + (n * axis.stride);
    let half_length = axis.tick_appearance_length / 2.0;
    let normal_y = normalize_y(&cplane.extent, axis.pos);
    let max_y = normal_y + half_length;
    let min_y = normal_y - half_length;
    while k <= cplane.extent.brect.x.end() {
        // TODO: Optimize
        let normal_x = normalize_x(&cplane.extent, k);
        let top = Vec2D { x: normal_x, y: max_y };
        let bot = Vec2D { x: normal_x, y: min_y };
        write_line_prefix(out, top, bot)?;
        write!(out, " stroke=\"black\"")?;
        write!(out, " ")?;
        gstyle.horizontal_axis_tick_attrs(out)?;
        write!(out, " ")?;
        gstyle.both_axis_tick_attrs(out)?;
        write!(out, "/>")?;        
        k += axis.stride;
    }
    return Ok(())
}


fn codegen_horizontal_axis_tick_labels<W>(out: &mut W, cplane: &CoordinatePlane,
    gstyle: &impl GlobalStyles, tex_renderer: &impl TeXRenderer) -> std::io::Result<()> 
where W: std::io::Write
{
    let Some(axis) = &cplane.horizontal_axis else { return Ok(()); };
    let Some(label) = &axis.tick_label else { return Ok(()); };
    if axis.stride == 0.0 { return Ok(()); }
    
    let vertical_axis_brect = calc_vertical_axis_brect(cplane, gstyle);
    
    write!(out, "<!-- horizontal axis tick labels begin -->")?;
    
    let n = ((cplane.extent.brect.x.begin() - axis.offset) / axis.stride).ceil();
    let y = normalize_y(&cplane.extent, axis.pos) + axis.tick_appearance_length;    
    let mut k = axis.offset + (n * axis.stride);
    let mut multiple: f64 = n;
    while k <= cplane.extent.brect.x.end() {
        let min_x = normalize_x(&cplane.extent, k - (0.5 * axis.stride));
        let max_x = normalize_x(&cplane.extent, k + (0.5 * axis.stride));
        let width = max_x - min_x;

        if let Some(brect) = vertical_axis_brect {
            if brect.includes(&Vec2D { x: (min_x + max_x) / 2.0, y }) {
                multiple += 1.0;
                k += axis.stride;
                continue;
            }
        }
        
        write!(out, "<svg")?;
        write!(out, " x=\"{}\"", min_x)?;
        write!(out, " y=\"{}\"", y)?;
        write!(out, " width=\"{}\"", width)?;
        write!(out, " height=\"{}\"", gstyle.font_height())?;
        write!(out, ">")?;
        match label {
            TickLabel::Decimal => tex_renderer.render_num(k, out, None)?,
            TickLabel::Symbolic(symbolic) => {
                let mut s = String::new();
                if let Some(offset_symbol_tex) = &symbolic.offset_symbol_tex {
                    s.push_str(&offset_symbol_tex);
                    s.push_str(" ");
                    if multiple >= 0.0 {
                        s.push_str("\\plus ");
                    }
                }
                if multiple == -1.0 {
                    s.push_str("-");
                }
                else if multiple != 1.0 {
                    s.push_str(&multiple.to_string());
                }
                s.push_str(" ");
                if multiple != 0.0 {  
                    s.push_str(&symbolic.stride_symbol_tex);
                }
                tex_renderer.render_str(&s, out, None)?;
            },
        }
        write!(out, "</svg>")?;
        k += axis.stride;
        multiple += 1.0;
    }
    return Ok(())
}


fn codegen_vertical_axis_tick_labels<W>(out: &mut W, cplane: &CoordinatePlane,
    gstyle: &impl GlobalStyles, tex_renderer: &impl TeXRenderer) -> std::io::Result<()> 
where W: std::io::Write
{
    let Some(axis) = &cplane.vertical_axis else { return Ok(()); };
    let Some(label) = &axis.tick_label else { return Ok(()); };
    if axis.stride == 0.0 { return Ok(()); }

    let horizontal_axis_brect = calc_horizontal_axis_brect(cplane, gstyle);

    write!(out, "<!-- vertical axis tick labels begin -->")?;
    
    let n = ((cplane.extent.brect.y.begin() - axis.offset) / axis.stride).ceil();
    let half_length = axis.tick_appearance_length / 2.0;
    let min_x = normalize_x(&cplane.extent, axis.pos) + half_length;

    let mut k = axis.offset + (n * axis.stride);
    let mut multiple = n;
    while k <= cplane.extent.brect.y.end() {
        let y = normalize_y(&cplane.extent, k);
        
        if let Some(brect) = horizontal_axis_brect {
            if brect.includes(&Vec2D { x: min_x , y }) {
                multiple += 1.0;
                k += axis.stride;
                continue;
            }
        }
        
        write!(out, "<svg")?;
        write!(out, " x=\"{}\"", min_x)?;
        write!(out, " y=\"{}\"", y - (0.5 * gstyle.font_height()))?;
        write!(out, " height=\"{}\"", gstyle.font_height())?;
        write!(out, ">")?;

        match label {
            TickLabel::Decimal => tex_renderer.render_num(k, out, Some("xMinYMin"))?,
            TickLabel::Symbolic(symbolic) => {
                let mut s = String::new();
                if let Some(offset_symbol_tex) = &symbolic.offset_symbol_tex {
                    s.push_str(&offset_symbol_tex);
                    s.push_str(" ");
                    if multiple >= 0.0 {
                        s.push_str("\\plus ");
                    }
                }
                if multiple == -1.0 {
                    s.push_str("-");
                }
                else if multiple != 1.0 {
                    s.push_str(&multiple.to_string());
                }
                s.push_str(" ");
                if multiple != 0.0 {  
                    s.push_str(&symbolic.stride_symbol_tex);
                }
                tex_renderer.render_str(&s, out, Some("xMinYMin"))?;
            },
        }
        write!(out, "</svg>")?;
        multiple += 1.0;
        k += axis.stride;
        
    }
    return Ok(())
}


fn codegen_vertical_axis_ticks<W>(out: &mut W, cplane: &CoordinatePlane, 
    gstyle: &impl GlobalStyles) -> std::io::Result<()> 
where W: std::io::Write
{
    let Some(axis) = &cplane.vertical_axis else { return Ok(()); };
    if axis.stride == 0.0 { return Ok(()); }
    let n = ((cplane.extent.brect.y.begin() - axis.offset) / axis.stride).ceil();
    let mut k = axis.offset + (n * axis.stride);
    let half_length = axis.tick_appearance_length / 2.0;
    let normal_x = normalize_x(&cplane.extent, axis.pos);
    let min_x = normal_x - half_length;
    let max_x = normal_x + half_length;
    while k <= cplane.extent.brect.y.end() {
        // TODO: Optimize
        let normal_y = normalize_y(&cplane.extent, k);
        let left = Vec2D { x: min_x, y: normal_y };
        let right = Vec2D { x: max_x, y: normal_y };
        write_line_prefix(out, left, right)?;
        write!(out, " stroke=\"black\"")?;
        write!(out, " ")?;
        gstyle.vertical_axis_tick_attrs(out)?;
        write!(out, " ")?;
        gstyle.both_axis_tick_attrs(out)?;
        write!(out, "/>")?;
        k += axis.stride;
    }
    return Ok(())
}


fn write_line_prefix<W>(out: &mut W, p1: Vec2D, p2: Vec2D) -> std::io::Result<()>
where W: std::io::Write
{
    write!(out, "<line")?;
    write!(out, " x1=\"{}\"", p1.x)?;
    write!(out, " y1=\"{}\"", p1.y)?;
    write!(out, " x2=\"{}\"", p2.x)?;
    write!(out, " y2=\"{}\"", p2.y)?;
    return Ok(())
}

fn normalize_coordinate(extent: &Extent, rel_coordinate: Vec2D) -> Vec2D {
    let x = normalize_x(extent, rel_coordinate.x);
    let y = normalize_y(extent, rel_coordinate.y);
    return Vec2D { x, y };
}

fn normalize_x(extent: &Extent, abs_x: f64) -> f64 {
    assert_ne!(extent.area(), 0.0, "container's interior is undefined and therefore so \
        is the interior coordinate system.");
    let delta_x = abs_x - extent.brect.x.begin();
    let maximum_dimension = f64::max(extent.brect.x.len(), extent.brect.y.len());
    return (delta_x / maximum_dimension) * extent.x_scale;
}

fn normalize_y(extent: &Extent, mut abs_y: f64) -> f64 {
    abs_y *= -1.0;
    let delta_y = abs_y - (-1.0 * extent.brect.y.end());
    let maximum_dimension = f64::max(extent.brect.x.len(), extent.brect.y.len());
    return (delta_y / maximum_dimension) * extent.y_scale;
}

fn calc_horizontal_axis_brect(cplane: &CoordinatePlane, gstyle: &impl GlobalStyles) 
-> Option<BoundingRect> 
{
    let Some(horizontal_axis) = &cplane.horizontal_axis else { return None; };
    let min_x = normalize_x(&cplane.extent, cplane.extent.brect.x.begin());
    let max_x = normalize_x(&cplane.extent, cplane.extent.brect.x.end());
    let y = normalize_y(&cplane.extent, horizontal_axis.pos);
    let min_y = y - (0.5 * horizontal_axis.tick_appearance_length);
    let max_y = y + (0.5 * horizontal_axis.tick_appearance_length) + gstyle.font_height();
    return Some(BoundingRect {
        x: ClosedInterval::new(NonDecreasing::new(min_x, max_x)),
        y: ClosedInterval::new(NonDecreasing::new(min_y, max_y))
    });
}

fn calc_vertical_axis_brect(cplane: &CoordinatePlane, gstyle: &impl GlobalStyles)
-> Option<BoundingRect>
{
    let Some(vertical_axis) = &cplane.vertical_axis else { return None; };
    let x = normalize_x(&cplane.extent, vertical_axis.pos);
    let half_tick_length = vertical_axis.tick_appearance_length * 0.5;
    let min_x = x - half_tick_length;
    let max_x = x + half_tick_length;
    let min_y = normalize_y(&cplane.extent, cplane.extent.brect.y.end());
    let max_y = normalize_y(&cplane.extent, cplane.extent.brect.y.begin());
    return Some(BoundingRect {
        x: ClosedInterval::new(NonDecreasing::new(min_x, max_x)),
        y: ClosedInterval::new(NonDecreasing::new(min_y, max_y))
    });
}
