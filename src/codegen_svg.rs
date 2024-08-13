// # SVG Code Generation

use crate::elements::{Function, CoordinatePlane, FunctionKind, Extent};
use crate::math::{self, BoundingRect, Vec2D};
use crate::plotfn::{self, PlotFnParams};
use crate::misc::{SegVec, SegVecRoot};

pub fn codegen_svg_cplane<W>(out: &mut W, cplane: &CoordinatePlane, gstyle: &impl SVGGlobalStyles<W>)
-> std::io::Result<()>
where W: std::io::Write
{ 
    if cplane.extent.area() == 0.0 { return Ok(()); }

    let mut bound = cplane.extent.brect.top_right();
    normalize_coordinate(&cplane.extent, &mut bound);
        
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
            codegen_svg_fnplot(out, &cplane.extent, function, buf.extend(), gstyle);
        }
    }
    codegen_svg_horizontal_axis(out, cplane, gstyle)?;
    codegen_svg_vertical_axis(out, cplane, gstyle)?;
    codegen_svg_horizontal_axis_ticks(out, cplane, gstyle)?;
    codegen_svg_vertical_axis_ticks(out, cplane, gstyle)?;
    write!(out, "</svg>")?;
    return Ok(())
}

pub trait SVGGlobalStyles<W> where W: std::io::Write {
    fn function_stroke_attrs(&self, dest: &mut W) -> std::io::Result<()> { Ok(()) }
    fn axis_stroke_attrs(&self, dest: &mut W) -> std::io::Result<()> { Ok(()) }
    fn horizontal_axis_tick_attrs(&self, dest: &mut W) -> std::io::Result<()> { Ok(()) }
    fn vertical_axis_tick_attrs(&self, dest: &mut W) -> std::io::Result<()> { Ok(()) }
    fn both_axis_tick_attrs(&self, destin: &mut W) -> std::io::Result<()> { Ok(()) }
    fn svg_root_attrs(&self, destin: &mut W) -> std::io::Result<()> { Ok(()) }
}

pub struct DefaultSVGGlobalStyles {
    pub function_stroke_width: f32,
    pub axis_stroke_width: f32,
    pub axis_tick_stroke_width: f32
}

impl DefaultSVGGlobalStyles {
    pub const DEFAULT_FUNCTION_STROKE_WIDTH: f32 = 1.0 / 400.0;
    pub const DEFAULT_AXIS_STROKE_WIDTH: f32 = 1.0 / 1000.0;
    pub const DEFAULT_AXIS_TICK_STROKE_WIDTH: f32 = 1.0 / 1000.0;
    
    pub fn new() -> Self {
        DefaultSVGGlobalStyles { 
            function_stroke_width: Self::DEFAULT_FUNCTION_STROKE_WIDTH,
            axis_stroke_width: Self::DEFAULT_AXIS_STROKE_WIDTH,
            axis_tick_stroke_width: Self::DEFAULT_AXIS_TICK_STROKE_WIDTH 
        }
    }
}

impl<W> SVGGlobalStyles<W> for DefaultSVGGlobalStyles where W: std::io::Write {
    fn function_stroke_attrs(&self, dest: &mut W) -> std::io::Result<()> {
        write!(dest, "stroke-width=\"{}\"", self.function_stroke_width)
    }

    fn axis_stroke_attrs(&self, dest: &mut W) -> std::io::Result<()> {
        write!(dest, "stroke-width=\"{}\"", self.axis_stroke_width)
    }

    fn horizontal_axis_tick_attrs(&self, dest: &mut W) -> std::io::Result<()> {
        write!(dest, "stroke-width=\"{}\"", self.axis_tick_stroke_width)
    }
    
    fn vertical_axis_tick_attrs(&self, dest: &mut W) -> std::io::Result<()> {
        write!(dest, "stroke-width=\"{}\"", self.axis_tick_stroke_width)
    }
}

fn codegen_svg_fnplot<W>(out: &mut W, extent: &Extent, function: &Function, 
    mut buf: SegVec<plotfn::Node>, gstyles: &impl SVGGlobalStyles<W>)
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

                let mut coord = Vec2D { x, y };
                normalize_coordinate(extent, &mut coord);
                write!(out, " {} {} ", coord.x, coord.y)?;
            },
        }
    }
    
    write!(out, "\"/>")?;
    return Ok(())
}

fn codegen_svg_vertical_axis<W>(out: &mut W, cplane: &CoordinatePlane, gstyle: &impl SVGGlobalStyles<W>) 
-> std::io::Result<()> 
where W: std::io::Write
{
    let Some(axis) = cplane.vertical_axis else { return Ok(()); };
    
    let mut start = Vec2D { 
        x: axis.pos,
        y: cplane.extent.brect.y.begin()
    };
    let mut stop = Vec2D { 
        x: axis.pos,
        y: cplane.extent.brect.y.end() 
    };

    normalize_coordinate(&cplane.extent, &mut start);
    normalize_coordinate(&cplane.extent, &mut stop);

    write_line_prefix(out, start, stop)?;
    write!(out, " stroke=\"black\"")?;
    write!(out, " ")?;
    gstyle.axis_stroke_attrs(out)?;
    write!(out, "/>")?;
    return Ok(());
}

fn codegen_svg_horizontal_axis<W>(out: &mut W, cplane: &CoordinatePlane, style: &impl SVGGlobalStyles<W>) 
-> std::io::Result<()> 
where W: std::io::Write
{
   let Some(axis) = cplane.horizontal_axis else { return Ok(()); };
    
    let mut start = Vec2D {
        x: cplane.extent.brect.x.begin(),
        y: axis.pos
    };
    let mut stop = Vec2D {
        x: cplane.extent.brect.x.end(),
        y: axis.pos
    };
    
    normalize_coordinate(&cplane.extent, &mut start);
    normalize_coordinate(&cplane.extent, &mut stop);

    write_line_prefix(out, start, stop)?;
    write!(out, " stroke=\"black\"")?;
    write!(out, " ")?;
    style.axis_stroke_attrs(out)?;
    write!(out, "/>")?;
    return Ok(());
}

fn codegen_svg_horizontal_axis_ticks<W>(out: &mut W, cplane: &CoordinatePlane, 
    gstyle: &impl SVGGlobalStyles<W>) -> std::io::Result<()> 
where W: std::io::Write
{
    let Some(axis) = cplane.horizontal_axis else { return Ok(()); };
    if axis.stride == 0.0 { return Ok(()); }
    // to find out how many ticks have elapsed, subtract the position of the first tick
    // from the begin point of the visible interval and divide that quantity by the number
    // by the distance between consecutive ticks.
    let n = ((cplane.extent.brect.x.begin() - axis.offset) / axis.stride).ceil();
    let mut k = axis.offset + (n * axis.stride);
    let half_length = axis.tick_appearance_length / 2.0;
    while k <= cplane.extent.brect.x.end() {
        let mut point = Vec2D { x: k, y: axis.pos };
        normalize_coordinate(&cplane.extent, &mut point);
        let mut top = point; top.y += half_length;
        let mut bottom = point; bottom.y -= half_length;
        write_line_prefix(out, top, bottom)?;
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


fn codegen_svg_vertical_axis_ticks<W>(out: &mut W, cplane: &CoordinatePlane, 
    gstyle: &impl SVGGlobalStyles<W>) -> std::io::Result<()> 
where W: std::io::Write
{
    let Some(axis) = cplane.vertical_axis else { return Ok(()); };
    if axis.stride == 0.0 { return Ok(()); }
    let n = ((cplane.extent.brect.y.begin() - axis.offset) / axis.stride).ceil();
    let mut k = axis.offset + (n * axis.stride);
    let half_length = axis.tick_appearance_length / 2.0;
    while k <= cplane.extent.brect.y.end() {
        let mut point = Vec2D { x: axis.pos, y: k };
        normalize_coordinate(&cplane.extent, &mut point);
        let mut left = point; left.x -= half_length;
        let mut right = point; right.x += half_length;
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
    

fn normalize_coordinate(extent: &Extent, coordinate: &mut Vec2D) {
    // The SVG coordinate system is the elementary coordinate system reflected across the x-axis.
    // Then, multiplying the y-coordinate by -1 transforms this coordinate into the SVG system.
    coordinate.y *= -1.0; 
    
    let mut svg_brect = extent.brect.clone();
    svg_brect.y.reflect();
    
    math::normalize_coordinate(&svg_brect, coordinate);    
    
    coordinate.x *= extent.x_scale;
    coordinate.y *= extent.y_scale;
}
