#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn max_value() -> Point {
        Point {
            x: std::i32::MAX,
            y: std::i32::MAX,
        }
    }

    pub fn min_value() -> Point {
        Point {
            x: std::i32::MIN,
            y: std::i32::MIN,
        }
    }

    pub fn min(self, other: &Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: &Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}
