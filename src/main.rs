
#![feature(box_syntax)]

extern crate nalgebra as na;
extern crate plotters;

use std::io::Write as IoWrite;

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

    let func = SimpleComputedFunction::find_solution(&MaterialVibration, 30);

    let points: Vec<(f32, f32)> = x_iter()
        .map(|x| {
            let y = func.evalute(x);

            (x as f32, y as f32)
        })
        .collect();

    let mut file = std::fs::File::create("points.csv")?;
    for (x, y) in &points {
        writeln!(&mut file, "{}, {}", x, y)?;
    }

    let backend = BitMapBackend::new("1.png", (1024, 1024));
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(25)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..2f32, -2.0f32..2f32)?;

    for base in &SimpleComputedFunction::get_bases(&MaterialVibration, 30) {
        use base_function::BaseFunction;

        let points = x_iter()
            .map(|x| {
                (x as f32, base.regular(x) as f32)
            });

        chart
            .draw_series(LineSeries::new(
                    points, 
                    &BLUE.mix(0.2),
            ))?;
    }

    let points2 = x_iter()
        .map(|x| {
            let y = 0.5f64 * (x * x.cos() + ((2f64.cos() + 2f64 * 2f64.sin())*x.sin())/(2f64.cos() - 2f64.sin()));
            (x as f32, y as f32)
        });

    chart
        .draw_series(LineSeries::new(
                points2, 
                &GREEN,
        ))?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
                points, 
                &RED,
        ))?;


    Ok(())
}
