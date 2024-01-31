use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::ops;

pub enum State {
    Collinear,
    Clock,
    CounterClock,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn like_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn abs(self) -> i32 {
        i32::pow(self.x * self.x + self.y * self.y, 1 / 2)
    }

    pub fn new() -> Point {
        Self { x: 0, y: 0 }
    }

    pub fn get_tuples(points: Vec<Point>) -> Vec<(i32, i32)> {
        let mut vec_of_tuples: Vec<(i32, i32)> = vec![];
        for point in points {
            vec_of_tuples.push(point.like_tuple());
        }
        vec_of_tuples
    }

    pub fn orientation(p: Point, q: Point, r: Point) -> State {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0 {
            State::Collinear
        } else if val > 0 {
            State::Clock
        } else {
            State::CounterClock
        }
    }
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = (rng.gen_range(-100i32..100i32), rng.gen_range(-100i32..100i32));
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<Point> for Point {
    type Output = i32;
    fn mul(self, rhs: Point) -> Self::Output {
        self.x * rhs.x + self.y * rhs.x
    }
}

impl ops::BitXor<Point> for Point {
    type Output = i32;
    fn bitxor(self, rhs: Point) -> Self::Output {
        self * rhs / self.abs() * rhs.abs()
    }
}
