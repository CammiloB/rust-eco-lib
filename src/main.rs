mod models;
mod utils;
mod simulator;

use plotters::prelude::*;

use simulator::Simulator;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let rate: f64 = 2.0;
    let n_cells: i32 = 10;
    let max_time: f64= 18.0;
    let doubling_time: f64 = 18.0;
    let sample_time: f64 =  0.1 * doubling_time;
    let growth_rate: f64 = rate.ln() / doubling_time;
    let base_size: f64 = 3.0;
    let total_steps: i32 = 2;
    let cv2_div = 0.001;
    let cv2_gr = 0.01;
    let mut simulator = Simulator::new(n_cells, growth_rate, base_size, total_steps, cv2_div, cv2_gr);
    simulator.initialize_cells();
    simulator.size_dynamics(max_time, sample_time);

    println!("{}", serde_json::to_string(&simulator.bacteriums)?);

    let root = BitMapBackend::new("examples/size_dynamics.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define the range for the x and y axes
    let mut chart = ChartBuilder::on(&root)
        .caption("Size Dynamics Chart", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

    // Configure the axes
    chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Size")
        .draw()?;

    let plot_data: Vec<(f64, f64)> = simulator.sizes.iter().map(|p| (p.time, p.size)).collect();

    chart.draw_series(LineSeries::new(plot_data.clone(), &BLUE))?
        .label("Vn")
        .legend(|(x, y)| {
            PathElement::new(vec![(x - 5, y - 5), (x + 5, y + 5)], &BLUE)
        });

    Ok(())

}


