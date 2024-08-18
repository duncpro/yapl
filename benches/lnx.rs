use criterion::{criterion_group, criterion_main, Criterion, black_box};

use yapl::elements::{CoordinatePlane, Function};
use yapl::style::Stylesheet;
use yapl::typography::NullTeXRenderer;
use yapl::codegen::codegen;

pub fn plot_lnx(c: &mut Criterion) {
    c.bench_function("plot_lnx", |b| b.iter(|| {
        let mut cplane = CoordinatePlane::new_minimal();

        let f = Function::new_elementary(|x| x.ln());
        cplane.fns.push(f);

        let stylesheet = Stylesheet::new_default();
        black_box(codegen(&mut yapl::misc::Dispose, &cplane, stylesheet, &mut NullTeXRenderer))
    }));
}

criterion_group!(benches, plot_lnx);
criterion_main!(benches);
