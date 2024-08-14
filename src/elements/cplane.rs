use crate::elements::axis::Axis;
use crate::elements::function::Function;
use crate::math::{BoundingRect, ClosedInterval, NonDecreasing};

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
    pub x_scale: f64,
    pub y_scale: f64
}

impl Extent {
    pub fn width(&self) -> f64 { self.x_scale * self.brect.x.len() }
    pub fn height(&self) -> f64 { self.y_scale * self.brect.y.len() }
    pub fn area(&self) -> f64 { self.width() * self.height() }
}
