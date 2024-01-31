use crate::point::{Point, State};

pub fn jarvismarch(points: Vec<Point>) -> Option<Vec<Point>> {
    if points.len() < 3 {
        panic!("not enough points!!!!!");
    }

    // find the leftmost point
    let mut leftmost = points[0].clone();
    for point in &points {
        if (point.x < leftmost.x) || ((point.x == leftmost.x) && (point.y < leftmost.y)) {
            leftmost = point.clone();
        }
    }
    let mut hull: Vec<Point> = Vec::new();

    let mut p = leftmost.clone();
    loop {
        hull.push(p);
        let index_of_p = points.iter().position(|&r| r == p).unwrap();
        let mut q = points[(index_of_p + 1) % points.len()];
        for point in &points {
            match Point::orientation(p, *point, q) {
                State::CounterClock => q = point.clone(),
                _ => (),
            }
        }
        p = q.clone();
        if p == leftmost {
            break;
        }
    }
    hull.push(hull[0].clone());
    Some(hull)
}
