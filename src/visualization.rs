use plotters::prelude::*;
use std::fs;
use std::collections::HashMap;

pub fn plot_histogram(
    degree_distribution: &std::collections::HashMap<usize, usize>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_path)?;
    let path = std::path::Path::new(output_path).join("degree_distribution.png");
    let root = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let max_degree = *degree_distribution.keys().max().unwrap_or(&1);
    let max_count = *degree_distribution.values().max().unwrap_or(&1);
    let mut chart = ChartBuilder::on(&root).caption("Degree Distribution", ("sans-serif", 40).into_font()).margin(10).x_label_area_size(30).y_label_area_size(40).build_cartesian_2d(0..max_degree, 0..max_count)?;
    chart.configure_mesh().draw()?;
    chart
        .draw_series(degree_distribution.iter().map(|(x, y)| {
            Rectangle::new(
                [
                    (if *x > 0 { x - 1 } else { *x }, 0),
                    (*x, *y),
                ],
                ShapeStyle {
                    color: BLUE.mix(0.8).to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
            )
        }))
        .map_err(|e| {
            eprintln!("Error drawing series: {}", e); 
            e 
        })?;
    root.present()?;
    println!("Histogram saved to {:?}", path);
    Ok(())
}

pub fn plot_log_log(
    degree_distribution: &HashMap<usize, usize>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(output_path)?;
    let path = std::path::Path::new(output_path).join("log_log_degree_distribution.png");
    let root = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let log_data: Vec<(f64, f64)> = degree_distribution.iter().filter(|(&x, &y)| x > 0 && y > 0) .map(|(&x, &y)| ((x as f64).ln(), (y as f64).ln())).collect();
    let max_x = log_data.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let max_y = log_data.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
    let mut chart = ChartBuilder::on(&root).caption("Log-Log Degree Distribution", ("sans-serif", 40).into_font()).margin(10).x_label_area_size(30).y_label_area_size(40).build_cartesian_2d(0.0..max_x, 0.0..max_y)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(log_data.iter().map(|(x, y)| {
        Circle::new((*x, *y), 3, BLUE.mix(0.8).filled())
    }))?;
    root.present()
        .map_err(|e| format!("Failed to present plot: {}", e))?;
    println!("Log-log plot saved to {:?}", path); 
    Ok(())
}
