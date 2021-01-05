
extern crate nalgebra as na;
extern crate plotters;

use plotters::prelude::*;

mod integration;
mod base_function;
mod fem;

use base_function::SimpleBaseFunction;

use fem::{
    ComputedFunction,
    MaterialVibration
};


fn x_iter() -> impl Iterator<Item = f64> {
    (0..2048)
        .map(|x| (x as f64) / 1024f64)
        .into_iter()
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    type SimpleComputedFunction = ComputedFunction::<SimpleBaseFunction>;

    let n = std::env::args()
        .nth(1)
        .and_then(|n| n.parse::<usize>().ok())
        .unwrap_or(20);

    let file = std::env::args()
        .nth(2)
        .unwrap_or("output.png".to_owned());

    let func = SimpleComputedFunction::find_solution(&MaterialVibration, n);

    let points: Vec<(f64, f64)> = x_iter()
        .map(|x| (x, func.evalute(x)))
        .collect();

    let backend = BitMapBackend::new(&file, (1024, 1024));
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(25)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..2f64, -2.0f64..2f64)?;

    if cfg!(feature = "debug-draw") {
        for base in &SimpleComputedFunction::get_bases(&MaterialVibration, n) {
            use base_function::BaseFunction;

            let points = x_iter()
                .map(|x| (x, base.regular(x)));

            chart
                .draw_series(LineSeries::new(
                        points, 
                        &BLUE.mix(0.2),
                ))?;
        }
        let points2 = x_iter()
            .map(|x| {
                let a: f64 = (2f64.cos() + 2f64 * 2f64.sin())/(2f64.cos() - 2f64.sin());
                let y = 0.5f64 * (x * x.cos() + a * x.sin());
                (x, y)
            });

        chart
            .draw_series(LineSeries::new(
                    points2, 
                    &GREEN,
            ))?;
    }


    chart
        .draw_series(LineSeries::new(
                points, 
                &RED,
        ))?;

    chart.configure_mesh().draw()?;


    Ok(())
}
