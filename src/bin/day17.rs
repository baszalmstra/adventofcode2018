use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Ground {
    Clay,
    Sand,
    Dried,
    Water,
    Source,
}

#[derive(Debug)]
struct Vein {
    from: (usize, usize),
    to: (usize, usize),
}

#[derive(Clone)]
struct World {
    tiles: Vec<Ground>,
    offset: (usize, usize),
    width: usize,
    height: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum Flow {
    Down,
    Blocked,
}

impl World {
    fn new(veins: Vec<Vein>) -> World {
        let min = veins
            .iter()
            .fold((usize::max_value(), usize::max_value()), |state, vein| {
                (state.0.min(vein.from.0 - 1), state.1.min(vein.from.1))
            });
        let max = veins
            .iter()
            .fold((usize::min_value(), usize::min_value()), |state, vein| {
                (state.0.max(vein.from.0 + 1), state.1.max(vein.from.1))
            });
        let width = max.0 - min.0 + 1;
        let height = max.1 + 1;
        let mut tiles = Vec::new();
        tiles.resize(width * height, Ground::Sand);
        let mut world = World {
            tiles,
            offset: (min.0, min.1),
            width,
            height,
        };
        world.set_tile((500, 0), Ground::Source);
        for vein in veins {
            for y in vein.from.1..=vein.to.1 {
                for x in vein.from.0..=vein.to.0 {
                    world.set_tile((x, y), Ground::Clay);
                }
            }
        }
        world
    }

    fn set_tile(&mut self, (x, y): (usize, usize), tile: Ground) {
        let index = y * self.width + (x - self.offset.0);
        if tile != Ground::Clay {
            assert_ne!(self.tiles[index], Ground::Clay);
        }
        self.tiles[index] = tile;
    }

    fn get_tile(&self, (x, y): (usize, usize)) -> Option<&Ground> {
        let index = y * self.width + (x - self.offset.0);
        self.tiles.get(index)
    }

    fn blocked(&self, position: (usize, usize)) -> bool {
        self.get_tile(position).map_or(false, |t| match t {
            Ground::Clay => true,
            Ground::Sand => false,
            Ground::Dried => false,
            Ground::Water => true,
            Ground::Source => false,
        })
    }

    fn flow(&mut self, (x, y): (usize, usize)) {
        // Move all the way down until we hit clay
        let bottom = {
            let (x, mut y) = (x, y);
            while y < self.height && !self.blocked((x, y + 1)) {
                self.set_tile((x, y), Ground::Dried);
                y += 1;
            }
            (x, y)
        };

        // If there is already water here we can be sure we've reached this place already.
        if let Some(Ground::Dried) = self.get_tile((bottom.0, bottom.1)) {
            return;
        }

        // Keep filling up rows until we no longer can
        let mut bottom_row = bottom.1;
        while bottom_row >= y && bottom_row < self.height {
            let (left, left_flow) = self.find_flow((x, bottom_row), -1);
            let (right, right_flow) = self.find_flow((x, bottom_row), 1);

            if left_flow == Flow::Blocked && right_flow == Flow::Blocked {
                for x in left..=right {
                    self.set_tile((x, bottom_row), Ground::Water);
                }
                bottom_row -= 1;
            } else {
                for x in left..=right {
                    self.set_tile((x, bottom_row), Ground::Dried);
                }
                if left_flow == Flow::Down && left >= self.offset.0 {
                    self.flow((left, bottom_row + 1));
                }
                if right_flow == Flow::Down && right < self.offset.0 + self.width {
                    self.flow((right, bottom_row + 1));
                }

                // Check if by flowing over edges we filled an entire container and we have to reevaluate this row.
                let (new_left, new_left_flow) = self.find_flow((x, bottom_row), -1);
                let (new_right, new_right_flow) = self.find_flow((x, bottom_row), 1);
                if new_left == left
                    && new_right == right
                    && new_left_flow == left_flow
                    && new_right_flow == right_flow
                {
                    return;
                }
            }
        }
    }

    fn find_flow(&self, (x, y): (usize, usize), direction: i32) -> (usize, Flow) {
        let mut pos = x;
        loop {
            if !self.blocked((pos, y + 1)) {
                return (pos, Flow::Down);
            }

            if self.blocked((((pos as i32) + direction) as usize, y)) {
                return (pos, Flow::Blocked);
            }

            pos = ((pos as i32) + direction) as usize;
        }
    }

    fn count_water_and_dried(&self) -> usize {
        let mut count = 0;
        for y in self.offset.1..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let ground = &self.tiles[index];
                if *ground == Ground::Dried || *ground == Ground::Water {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_water(&self) -> usize {
        let mut count = 0;
        for y in self.offset.1..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let ground = &self.tiles[index];
                if *ground == Ground::Water {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = &self.tiles[y * self.width + x];
                write!(
                    f,
                    "{}",
                    match c {
                        Ground::Sand => ".",
                        Ground::Clay => "#",
                        Ground::Dried => "|",
                        Ground::Water => "~",
                        Ground::Source => "+",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day17/input").expect("Could not read input file");
    let regex = regex::Regex::new(r"([xy])=(\d+), ([xy])=(\d+)..(\d+)").unwrap();
    let veins: Vec<Vein> = input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            if &captures[1] == "x" {
                Vein {
                    from: (captures[2].parse().unwrap(), captures[4].parse().unwrap()),
                    to: (captures[2].parse().unwrap(), captures[5].parse().unwrap()),
                }
            } else {
                Vein {
                    from: (captures[4].parse().unwrap(), captures[2].parse().unwrap()),
                    to: (captures[5].parse().unwrap(), captures[2].parse().unwrap()),
                }
            }
        })
        .collect();

    let mut world = World::new(veins);
    world.flow((500, 1));

    println!("{}", world);
    println!("Result 1: {}", world.count_water_and_dried());
    println!("Result 2: {}", world.count_water());
}
