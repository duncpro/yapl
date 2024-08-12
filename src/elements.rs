use crate::math::{NonDecreasing, BoundingRect, ClosedInterval};

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
    pub unit: f32,
    pub offset: f32, 
    pub pos: f32 
}

pub struct CoordinatePlane {
    pub extent: BoundingRect,
    pub horizontal_axis: Option<Axis>,
    pub vertical_axis: Option<Axis>,
    pub fns: Vec<Function>
}


impl CoordinatePlane {
    pub fn new_elementary() -> Self {
        CoordinatePlane { 
            extent: BoundingRect { 
                x: ClosedInterval::new(NonDecreasing::new(-5.0, 5.0)), 
                y: ClosedInterval::new(NonDecreasing::new(-5.0, 5.0))
            },
            horizontal_axis: Some(Axis { unit: 1.0, offset: 0.0, pos: 0.0 }),
            vertical_axis: Some(Axis { unit: 1.0, offset: 0.0, pos: 0.0}), 
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
