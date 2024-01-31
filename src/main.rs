use rand::Rng;
mod jarvis;
mod point;
use jarvis::jarvismarch;
use plotters::prelude::*;
use point::Point;

fn generate_random_points(count_of_points: i32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<Point> = Vec::new();
    for _ in 0..count_of_points {
        points.push(rng.gen::<Point>())
    }
    points
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("res/5.png", (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption("Jarvis", ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(-100i32..100i32, -100i32..100i32)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    let points = generate_random_points(3000);

    chart.draw_series(PointSeries::of_element(
        Point::get_tuples(points.clone()),
        2,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)
                + Circle::new((0, 0), s, st.filled())
                + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;

    let pts = jarvismarch(points.clone());
    println!("{:#?}", pts);
    chart.draw_series(LineSeries::new(Point::get_tuples(pts.unwrap()), &RED))?;

    root.present()?;
    Ok(())
}
