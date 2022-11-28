use std::cmp::PartialEq;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Add<Output=T> + Sub<Output=T> + Eq> Point2<T> {
    pub fn new(x: T, y: T) -> Point2<T> where T: Add {
        Point2 {
            x,
            y,
        }
    }

    pub fn sum(a: Point2<T>, b: Point2<T>) -> Point2<T> where T: Sub {
        Point2 {
            x: a.x + b.x,
            y: a.y + b.y,
        }
    }

    pub fn diff(a: Point2<T>, b: Point2<T>) -> Point2<T> {
        Point2 {
            x: a.x - b.x,
            y: a.y - b.y,
        }
    }
}

pub type IPoint2 = Point2<i32>;

impl IPoint2 {
    pub fn warp(&mut self, rows: i32, cols: i32) {
        self.x = self.x.rem_euclid(cols);
        self.y = self.y.rem_euclid(rows);
    }
}

impl PartialEq for IPoint2 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}