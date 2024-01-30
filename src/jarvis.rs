use crate::point::{Point, State};

pub fn jarvismarch(points: Vec<Point>) -> Option<Vec<Point>> {
    if points.len() < 3 {
        panic!("not enough points!!!!!");
    }

    // find the leftmost point
    let mut p0 = points[0].clone();
    for point in &points {
        if (point.x < p0.x) || ((point.x == p0.x) && (point.y < p0.y)) {
            p0 = point.clone();
        }
    }
    let mut hull: Vec<Point> = Vec::new();

    let mut p = p0.clone();
    loop {
        hull.push(p);
        let index_of_p = points.iter().position(|&r| r == p).unwrap();
        let mut q = points[(index_of_p + 1) % points.len()];
        for i in 0..points.len() {
            match Point::orientation(p, points[i], q) {
                State::CounterClock => q = points[i].clone(),
                _ => (),
            }
        }
        p = q.clone();
        if p == p0 {
            break;
        }
    }
    hull.push(hull[0].clone());
    Some(hull)
}
