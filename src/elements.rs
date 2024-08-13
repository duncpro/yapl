use crate::math::{NonDecreasing, BoundingRect, ClosedInterval, Vec2D};

pub enum FunctionKind { OfX, OfY }

pub struct Function {
    pub eval: Box<dyn Fn(f32) -> f32>,

    /// See `min_depth` in [`crate::plotfn::PlotFnParams`].
    pub min_depth: usize,

    /// See `error_tolerance` in [`crate::plotfn::PlotFnParams`].
    ///
    /// Specifically `error_tolerance = codomain_length / error_tolerance_factor`.
    pub error_tolerance_factor: f32,

    /// See `zero_tolerance` in [`crate::plotfn::PlotFnParams`]
    ///
    /// Specifically `zero_tolerance = domain_length / zero_tolerance_factor`.
    pub zero_tolerance_factor: f32,

    pub kind: FunctionKind
}

impl Function {
    pub const DEFAULT_MIN_DEPTH: usize = 1;
    /// Based on the average display width in physical pixels.
    pub const DEFAULT_ERROR_TOLERANCE_FACTOR: f32 = 2000.0;    
    /// Based on the average display width in physical pixels. 
    pub const DEFAULT_ZERO_TOLERANCE_FACTOR: f32 = 2000.0;
    pub const DEFAULT_KIND: FunctionKind = FunctionKind::OfX;
    
    pub fn new_default<F>(f: F) -> Self
    where F: Fn(f32) -> f32 + 'static
    {
        Function { 
            eval: Box::new(f),
            min_depth: Self::DEFAULT_MIN_DEPTH, 
            error_tolerance_factor: Self::DEFAULT_ERROR_TOLERANCE_FACTOR, 
            zero_tolerance_factor: Self::DEFAULT_ZERO_TOLERANCE_FACTOR, 
            kind: Self::DEFAULT_KIND 
        }
    }
}

#[derive(Clone, Copy)]
pub struct Axis { 
    pub offset: f32, 
    pub stride: f32,
    pub pos: f32,
    pub tick_appearance_length: f32
}

impl Axis {
    pub const DEFAULT_TICK_APPEARANCE_LENGTH: f32 = 1.0 / 100.0;

    /// Creates a new [`Axis`] with the default `tick_appearance_length`.
    pub fn new_default(offset: f32, stride: f32, pos: f32) -> Self {
        Self { offset, stride, pos, 
            tick_appearance_length: Self::DEFAULT_TICK_APPEARANCE_LENGTH }
    }
}

pub struct CoordinatePlane {
    pub extent: Extent,
    pub horizontal_axis: Option<Axis>,
    pub vertical_axis: Option<Axis>,
    pub fns: Vec<Function>
}


impl CoordinatePlane {
    pub fn new_elementary() -> Self {
        CoordinatePlane { 
            extent: Extent {
                brect: BoundingRect { 
                    x: ClosedInterval::new(NonDecreasing::new(-5.0, 5.0)), 
                    y: ClosedInterval::new(NonDecreasing::new(-5.0, 5.0))
                },  
                x_scale: 1.0,
                y_scale: 1.0          
            },
            horizontal_axis: Some(Axis::new_default(0.0, 1.0, 0.0)),
            vertical_axis: Some(Axis::new_default(0.0, 1.0, 0.0)), 
            fns: Vec::new()
        }
    }

    pub fn new_minimal() -> Self {
        CoordinatePlane { 
            horizontal_axis: None,
            vertical_axis: None, 
            ..Self::new_elementary()
        }
    }
}

pub struct Extent {
    pub brect: BoundingRect,
    pub x_scale: f32,
    pub y_scale: f32
}

impl Extent {
    pub fn width(&self) -> f32 { self.x_scale * self.brect.x.len() }
    pub fn height(&self) -> f32 { self.y_scale * self.brect.y.len() }
    pub fn area(&self) -> f32 { self.width() * self.height() }
}
