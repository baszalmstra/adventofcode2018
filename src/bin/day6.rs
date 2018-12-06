use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Copy, Clone, Debug)]
enum VoronoiCell {
    Uninitialized,
    ClosestTo(usize, u32),
    MultipleClosest
}

impl Point {
    pub fn new(x:u32, y:u32) -> Point {
        Point {x,y}
    }

    pub fn max_value() -> Point {
        Point {
            x: std::u32::MAX,
            y: std::u32::MAX
        }
    }

    pub fn min_value() -> Point {
        Point {
            x: std::u32::MIN,
            y: std::u32::MIN
        }
    }

    pub fn min(&self, other:&Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y)
        }
    }

    pub fn max(&self, other:&Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y)
        }
    }
}

fn main() {
    // Parse the input
    let input:Vec<Point> = std::fs::read_to_string("inputs/day6/input")
        .expect("Could not read input file")
        .lines()
        .map(|l| {
            let mut coords = l.split(", ");
            Point::new(
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap())
        })
        .collect();

    // Find the bounds of the points
    let min = input.iter().fold(Point::max_value(), |s, v| s.min(v));
    let max = input.iter().fold(Point::min_value(), |s, v| s.max(v));

    // Build a voronoi by flood filling a grid
    let width = max.x - min.x + 1;
    let height = max.y - min.y + 1;
    let mut voronoi:Vec<VoronoiCell> = Vec::new();
    voronoi.resize((width*height) as usize, VoronoiCell::Uninitialized);

    let mut queue = VecDeque::new();
    for (idx, point) in input.iter().enumerate() {
        queue.push_back((0, idx, *point));
    }

    while let Some(item) = queue.pop_front() {
        let idx = ((item.2.y - min.y) * width + (item.2.x - min.x)) as usize;
        let location = item.2;
        match voronoi[idx] {
            VoronoiCell::Uninitialized => {
                voronoi[idx] = VoronoiCell::ClosestTo(item.1, item.0);
            },
            VoronoiCell::ClosestTo(index, distance) => {
                if distance == item.0 {
                    if index == item.1 {
                        continue;
                    } else {
                        voronoi[idx] = VoronoiCell::MultipleClosest;
                    }
                } else if distance >= item.0 {
                    voronoi[idx] = VoronoiCell::ClosestTo(item.1, item.0);
                } else {
                    continue;
                }
            },
            VoronoiCell::MultipleClosest => continue,
        }

        // Add the neighbours to the queue
        if location.x > min.x {
            queue.push_back((item.0 + 1, item.1, Point::new(location.x - 1, location.y)))
        }
        if location.x < max.x {
            queue.push_back((item.0 + 1, item.1, Point::new(location.x + 1, location.y)))
        }
        if location.y > min.y {
            queue.push_back((item.0 + 1, item.1, Point::new(location.x, location.y-1)))
        }
        if location.y < max.y {
            queue.push_back((item.0 + 1, item.1, Point::new(location.x, location.y+1)))
        }
    }

    let mut areas = Vec::new();
    areas.resize(input.len(), 0);
    for cell in voronoi {
        if let VoronoiCell::ClosestTo(idx,_) = cell {
            areas[idx] += 1;
        }
    }

    let largest_finite_area = areas.iter()
        .enumerate()
        .filter(|(idx, _)| input[*idx].x > min.x && input[*idx].x < max.x && input[*idx].y > min.y && input[*idx].y < max.y)
        .max_by(|(_,a), (_,b)| a.cmp(b))
        .unwrap();

    println!("Result 1: {}", largest_finite_area.1);

    let mut count = 0;
    for y in min.y .. max.y + 1 {
        for x in min.x .. max.x + 1 {
            let mut total_distance = 0;
            for point in input.iter() {
                let distance = (x as i64 - point.x as i64).abs() + (y as i64 - point.y as i64).abs();
                total_distance += distance;
                if total_distance >= 10000 {
                    break
                }
            }

            if total_distance < 10000 {
                count += 1;
            }
        }
    }

    println!("Result 2: {}", count);
}