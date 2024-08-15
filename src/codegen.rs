// # SVG Code Generation

use crate::elements::axis::{AxisDefaultStyleClass, write_axis_default_style_class};
use crate::elements::axis::{TickDefaultStyleClass, write_tick_default_style_class};
use crate::elements::function::{FunctionDefaultStyleClass, write_function_default_style_class};
use crate::elements::{CoordinatePlane, Extent, TickLabelKind, Function, FunctionKind};
use crate::math::{Vec2D, BoundingRect, ClosedInterval, NonDecreasing};
use crate::plotfn::{self, PlotFnParams};
use crate::misc::{SegVec, SegVecRoot};
use crate::typography::TeXRenderer;
use crate::style::Stylesheet;

pub fn codegen<T, W>(out: &mut W, cplane: &CoordinatePlane, stylesheet: Stylesheet, tex_renderer: &mut T)
-> std::io::Result<()> 
where T: TeXRenderer, W: std::io::Write
{
    let mut ctx = CodegenCtx { stylesheet, tex_renderer, out };
    ctx.codegen_cplane(cplane)?;
    return Ok(());
}


struct CodegenCtx<'a, T, W> 
where T: TeXRenderer, 
      W: std::io::Write
{ 
    stylesheet: Stylesheet<'a>,
    tex_renderer: &'a mut T,
    out: &'a mut W
}

impl<'a, T, W> CodegenCtx<'a, T, W> 
where T: TeXRenderer,
      W: std::io::Write
{
    fn codegen_stylesheet(&mut self, cplane: &CoordinatePlane) -> std::io::Result<()> {
        let def_fn_count = cplane.fns.iter().filter(|f| f.apply_default_style_class).count();
        let mut def_axis_count = 0usize;
        let mut def_tick_count = 0usize;
        if let Some(axis) = &cplane.horizontal_axis {
            if axis.apply_default_style_class {
                def_axis_count += 1;
            }
            if axis.tick.apply_default_style_class {
                def_tick_count += 1;
            }
        }
        if let Some(axis) = &cplane.vertical_axis {
            if axis.apply_default_style_class {
                def_axis_count += 1;
            }
            if axis.tick.apply_default_style_class {
                def_tick_count += 1;
            }
        }
        let total_def = def_fn_count + def_axis_count + def_tick_count;

        // In this case we need not print a style tag at all.
        if total_def == 0 && self.stylesheet.custom.is_none() { return Ok(()); };

        write!(self.out, "<style>")?;
        write!(self.out, "<![CDATA[")?;
        if def_fn_count > 0 {
            write_function_default_style_class(self.out, &self.stylesheet.defaults.function)?;
        }
        if def_axis_count > 0 {
            write_axis_default_style_class(self.out, &self.stylesheet.defaults.axis)?;
        }
        if def_tick_count > 0 {
            write_tick_default_style_class(self.out, &self.stylesheet.defaults.tick)?;
        }
        if let Some(custom_styles) = self.stylesheet.custom {
            write!(self.out, "{}", custom_styles)?;
        }
        write!(self.out, "]]>")?;
        write!(self.out, "</style>")?;
        
        return Ok(())
    }
    
    fn codegen_cplane(&mut self, cplane: &CoordinatePlane) 
    -> std::io::Result<()>
    { 
        if cplane.extent.area() == 0.0 { return Ok(()); }
        let bound = normalize_coordinate(&cplane.extent, cplane.extent.brect.top_right());
    
        write!(self.out, "<svg")?;
        write!(self.out, " viewBox=\"0 0 {} {}\"", bound.x, bound.y)?;
        write!(self.out, " xmlns=\"http://www.w3.org/2000/svg\"")?;
        write!(self.out, " preserveAspectRatio=\"xMinYMin meet\"")?;
        write!(self.out, ">")?;    
        self.codegen_stylesheet(cplane)?;
        {
            let mut buf: SegVecRoot<plotfn::Node> = SegVecRoot::default();
            for function in &cplane.fns {
                self.codegen_fnplot(&cplane.extent, function, buf.extend())?;
            }
        }
        self.codegen_horizontal_axis(cplane)?;
        self.codegen_vertical_axis(cplane)?;
        self.codegen_horizontal_axis_ticks(cplane)?;
        self.codegen_vertical_axis_ticks(cplane)?;
        self.codegen_horizontal_axis_tick_labels(cplane)?;
        self.codegen_vertical_axis_tick_labels(cplane)?;
        write!(self.out, "</svg>")?;
        return Ok(())
    }

    
    fn codegen_fnplot(&mut self, extent: &Extent, function: &Function, mut buf: SegVec<plotfn::Node>) 
    -> std::io::Result<()>
    {    
        let (domain, codomain) = match function.kind {
            FunctionKind::OfX => (extent.brect.x, extent.brect.y),
            FunctionKind::OfY => (extent.brect.y, extent.brect.x),
        };
    
        let error_tolerance = codomain.len() / function.error_tolerance_factor;
        let zero_tolerance = domain.len() / function.zero_tolerance_factor;
                
        plotfn::plotfn(&function.eval, &mut buf, PlotFnParams { domain, codomain, 
            min_depth: function.min_depth, error_tolerance, zero_tolerance }); 
           
        write!(self.out, "<path")?;
        write!(self.out, " class=\"")?;
        if function.apply_default_style_class {
            write!(self.out, " {}", FunctionDefaultStyleClass::NAME)?;
        }
        if let Some(class) = function.style_class { write!(self.out, " {}", class)?; }
        write!(self.out, "\"")?;
        write!(self.out, " d=\"")?;
    
        let mut broken = true;
        for node in buf.as_slice().iter() {
            match node {
                plotfn::Node::Break => broken = true,
                plotfn::Node::Anchor(anchor) => {
                    match broken {
                        true => write!(self.out, "M")?,
                        false => write!(self.out, "L")?,
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
                    write!(self.out, " {} {} ", coord.x, coord.y)?;
                },
            }
        }
        
        write!(self.out, "\"/>")?;
        return Ok(())
    }

    fn codegen_vertical_axis(&mut self, cplane: &CoordinatePlane) -> std::io::Result<()> 
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
    
        write_line_prefix(self.out, start, stop)?;
        write!(self.out, " class=\"")?;
        if axis.apply_default_style_class {
            write!(self.out, " {}", AxisDefaultStyleClass::NAME)?;
        }
        if let Some(class) = axis.style_class {
            write!(self.out, " {}", class)?;
        }
        write!(self.out, "\"")?;
        write!(self.out, "/>")?;
        return Ok(());
    }
    
    fn codegen_horizontal_axis(&mut self, cplane: &CoordinatePlane) -> std::io::Result<()> 
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
    
        write_line_prefix(self.out, start, stop)?;
        write!(self.out, " class=\"")?;
        if axis.apply_default_style_class {
            write!(self.out, " {}", AxisDefaultStyleClass::NAME)?;
        }
        if let Some(class) = axis.style_class {
            write!(self.out, " {}", class)?;
        }
        write!(self.out, "\"")?;
        write!(self.out, "/>")?;
        return Ok(());
    }


    fn codegen_vertical_axis_ticks(&mut self, cplane: &CoordinatePlane) -> std::io::Result<()> 
    {
        let Some(axis) = &cplane.vertical_axis else { return Ok(()); };
        if axis.stride == 0.0 { return Ok(()); }
        let n = ((cplane.extent.brect.y.begin() - axis.offset) / axis.stride).ceil();
        let mut k = axis.offset + (n * axis.stride);
        let half_length = axis.tick.len / 2.0;
        let normal_x = normalize_x(&cplane.extent, axis.pos);
        let min_x = normal_x - half_length;
        let max_x = normal_x + half_length;
        while k <= cplane.extent.brect.y.end() {
            // TODO: Optimize
            let normal_y = normalize_y(&cplane.extent, k);
            let left = Vec2D { x: min_x, y: normal_y };
            let right = Vec2D { x: max_x, y: normal_y };
            write_line_prefix(self.out, left, right)?;
            write!(self.out, " class=\"")?;
            if axis.tick.apply_default_style_class {
                write!(self.out, " {}", TickDefaultStyleClass::NAME)?;
            }
            if let Some(class) = axis.tick.style_class {
                write!(self.out, " {}", class)?;
            }
            write!(self.out, "\"")?;
            write!(self.out, "/>")?;
            k += axis.stride;
        }
        return Ok(())
    }

    fn codegen_horizontal_axis_ticks(&mut self, cplane: &CoordinatePlane) -> std::io::Result<()> 
    where W: std::io::Write
    {
        let Some(axis) = &cplane.horizontal_axis else { return Ok(()); };
        if axis.stride == 0.0 { return Ok(()); }
        let n = ((cplane.extent.brect.x.begin() - axis.offset) / axis.stride).ceil();
        let mut k = axis.offset + (n * axis.stride);
        let half_length = axis.tick.len / 2.0;
        let normal_y = normalize_y(&cplane.extent, axis.pos);
        let max_y = normal_y + half_length;
        let min_y = normal_y - half_length;
        while k <= cplane.extent.brect.x.end() {
            // TODO: Optimize
            let normal_x = normalize_x(&cplane.extent, k);
            let top = Vec2D { x: normal_x, y: max_y };
            let bot = Vec2D { x: normal_x, y: min_y };
            write_line_prefix(self.out, top, bot)?;

            write!(self.out, " class=\"")?;
            if axis.tick.apply_default_style_class {
                write!(self.out, " {}", TickDefaultStyleClass::NAME)?;
            }
            if let Some(class) = axis.tick.style_class {
                write!(self.out, " {}", class)?;
            }
            write!(self.out, "\"")?;
            write!(self.out, "/>")?;
            k += axis.stride;
        }
        return Ok(())
    }

    
    fn codegen_horizontal_axis_tick_labels(&mut self, cplane: &CoordinatePlane)
    -> std::io::Result<()> 
     {
        let Some(axis) = &cplane.horizontal_axis else { return Ok(()); };
        let Some(label) = &axis.tick_label else { return Ok(()); };
        if axis.stride == 0.0 { return Ok(()); }
        
        let vertical_axis_brect = calc_vertical_axis_brect(cplane);
        
        write!(self.out, "<!-- horizontal axis tick labels begin -->")?;
        
        let n = ((cplane.extent.brect.x.begin() - axis.offset) / axis.stride).ceil();
        let y = normalize_y(&cplane.extent, axis.pos) + axis.tick.len;    
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
            
            write!(self.out, "<svg")?;
            write!(self.out, " x=\"{}\"", min_x)?;
            write!(self.out, " y=\"{}\"", y)?;
            write!(self.out, " width=\"{}\"", width)?;
            write!(self.out, " height=\"{}\"", label.typography_height)?;
            write!(self.out, ">")?;
            match label.kind {
                TickLabelKind::Decimal => self.tex_renderer.render_num(k, self.out, None)?,
                TickLabelKind::Symbolic(symbolic) => {
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
                    self.tex_renderer.render_str(&s, self.out, None)?;
                },
            }
            write!(self.out, "</svg>")?;
            k += axis.stride;
            multiple += 1.0;
        }
        return Ok(())
    }

    
    fn codegen_vertical_axis_tick_labels(&mut self, cplane: &CoordinatePlane)
    -> std::io::Result<()> 
    {
        let Some(axis) = &cplane.vertical_axis else { return Ok(()); };
        let Some(label) = &axis.tick_label else { return Ok(()); };
        if axis.stride == 0.0 { return Ok(()); }
    
        let horizontal_axis_brect = calc_horizontal_axis_brect(cplane);
    
        write!(self.out, "<!-- vertical axis tick labels begin -->")?;
        
        let n = ((cplane.extent.brect.y.begin() - axis.offset) / axis.stride).ceil();
        let half_length = axis.tick.len / 2.0;
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
            
            write!(self.out, "<svg")?;
            write!(self.out, " x=\"{}\"", min_x)?;
            write!(self.out, " y=\"{}\"", y - (0.5 * label.typography_height))?;
            write!(self.out, " height=\"{}\"", label.typography_height)?;
            write!(self.out, ">")?;
    
            match label.kind {
                TickLabelKind::Decimal => self.tex_renderer.render_num(k, self.out, Some("xMinYMin"))?,
                TickLabelKind::Symbolic(symbolic) => {
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
                    self.tex_renderer.render_str(&s, self.out, Some("xMinYMin"))?;
                },
            }
            write!(self.out, "</svg>")?;
            multiple += 1.0;
            k += axis.stride;
        }
        return Ok(())
    }
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

fn calc_horizontal_axis_brect(cplane: &CoordinatePlane) 
-> Option<BoundingRect> 
{
    let Some(horizontal_axis) = &cplane.horizontal_axis else { return None; };
    let min_x = normalize_x(&cplane.extent, cplane.extent.brect.x.begin());
    let max_x = normalize_x(&cplane.extent, cplane.extent.brect.x.end());
    let y = normalize_y(&cplane.extent, horizontal_axis.pos);
    let min_y = y - (0.5 * horizontal_axis.tick.len);
    let mut max_y = y + (0.5 * horizontal_axis.tick.len);
    if let Some(label) = horizontal_axis.tick_label {
        max_y += label.typography_height;
    }
    return Some(BoundingRect {
        x: ClosedInterval::new(NonDecreasing::new(min_x, max_x)),
        y: ClosedInterval::new(NonDecreasing::new(min_y, max_y))
    });
}

fn calc_vertical_axis_brect(cplane: &CoordinatePlane)
-> Option<BoundingRect>
{
    let Some(vertical_axis) = &cplane.vertical_axis else { return None; };
    let x = normalize_x(&cplane.extent, vertical_axis.pos);
    let half_tick_length = vertical_axis.tick.len * 0.5;
    let min_x = x - half_tick_length;
    let max_x = x + half_tick_length;
    let min_y = normalize_y(&cplane.extent, cplane.extent.brect.y.end());
    let max_y = normalize_y(&cplane.extent, cplane.extent.brect.y.begin());
    return Some(BoundingRect {
        x: ClosedInterval::new(NonDecreasing::new(min_x, max_x)),
        y: ClosedInterval::new(NonDecreasing::new(min_y, max_y))
    });
}

