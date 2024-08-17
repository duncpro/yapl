use criterion::{criterion_group, criterion_main, Criterion};

use yapl::elements::{CoordinatePlane, Function};
use yapl::math::{NonDecreasing, ClosedInterval};
use yapl::style::Stylesheet;
use yapl::typography::MathJaxProcessTeXRenderer;
use yapl::codegen::codegen;

pub fn plot_1oversinx(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| {
        let mut cplane = CoordinatePlane::new_minimal();
        cplane.extent.brect.x = ClosedInterval::new(NonDecreasing::new(-0.5, 0.5));
        cplane.extent.brect.y = ClosedInterval::new(NonDecreasing::new(-1.1, 1.1));
        cplane.extent.x_scale = 8.0;

        let mut f = Function::new_default(|x| (1.0 / x).sin());
        f.zero_tolerance_factor = 10.0f64.powi(7);
        cplane.fns.push(f);

        let mut out = yapl::misc::Dispose;
        
        let mut tex_renderer = MathJaxProcessTeXRenderer::new()?;
        let stylesheet = Stylesheet::new_default();
        codegen(&mut out, &cplane, stylesheet, &mut tex_renderer)
    }));
}

criterion_group!(benches, plot_1oversinx);
criterion_main!(benches);
