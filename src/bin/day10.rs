use aoc::Point;

fn main() {
    let regex =
        regex::Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
            .unwrap();
    let input: Vec<(Point, Point)> = std::fs::read_to_string("inputs/day10/input")
        .expect("Could not read input file")
        .lines()
        .map(|l| {
            let captures = regex.captures(l).unwrap();
            (
                Point::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                Point::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            )
        })
        .collect();

    // Find the initial bounds
    let bounds = input
        .iter()
        .fold((Point::max_value(), Point::min_value()), |s, v| {
            (s.0.min(&v.0), s.1.max(&v.0))
        });

    // Iterate over the points to find the inflection point where the height no longer shrinks
    let mut height = bounds.1.y - bounds.0.y;
    let mut seconds = 1;
    loop {
        let bounds = input
            .iter()
            .map(|(p, v)| Point::new(p.x + v.x * seconds, p.y + v.y * seconds))
            .fold((Point::max_value(), Point::min_value()), |s, v| {
                (s.0.min(&v), s.1.max(&v))
            });

        let current_height = bounds.1.y - bounds.0.y;
        if current_height > height {
            seconds -= 1;
            break;
        }

        height = current_height;
        seconds += 1;
    }

    let bounds = input
        .iter()
        .map(|(p, v)| Point::new(p.x + v.x * seconds, p.y + v.y * seconds))
        .fold((Point::max_value(), Point::min_value()), |s, v| {
            (s.0.min(&v), s.1.max(&v))
        });

    let width = bounds.1.x - bounds.0.x + 1;
    let height = bounds.1.y - bounds.0.y + 1;
    let mut grid = Vec::with_capacity((width * height) as usize);
    grid.resize((width * height) as usize, false);

    for (p, v) in input {
        let y = p.y + v.y * seconds - bounds.0.y;
        let x = p.x + v.x * seconds - bounds.0.x;
        let idx = y * width + x;
        grid[idx as usize] = true;
    }

    println!("Result 1:");
    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                if grid[(y * width + x) as usize] {
                    "#"
                } else {
                    " "
                }
            );
        }
        println!();
    }

    println!("Result 2: {}", seconds);
}
