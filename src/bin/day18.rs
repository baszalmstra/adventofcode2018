use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
enum GroundType {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct World {
    area: Vec<GroundType>,
}

impl World {
    fn new() -> World {
        let mut area = Vec::new();
        area.resize(50 * 50, GroundType::Open);
        World { area }
    }

    fn adjacent(pos: usize) -> impl Iterator<Item = usize> {
        let x = (pos % 50) as i64;
        let y = ((pos as i64 - x) / 50) as i64;
        static SQUARES: [(i64, i64); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        SQUARES
            .iter()
            .map(move |(dx, dy)| (*dx + x, *dy + y))
            .filter(|(x, y)| *x >= 0 && *x < 50 && *y >= 0 && *y < 50)
            .map(|(x, y)| (y * 50 + x) as usize)
    }

    fn evolve(self) -> World {
        let mut new_world = World::new();
        for idx in 0..50 * 50 {
            new_world.area[idx] = match self.area[idx] {
                GroundType::Open => {
                    if World::adjacent(idx)
                        .filter(|i| self.area[*i] == GroundType::Trees)
                        .count()
                        >= 3
                    {
                        GroundType::Trees
                    } else {
                        GroundType::Open
                    }
                }
                GroundType::Trees => {
                    if World::adjacent(idx)
                        .filter(|i| self.area[*i] == GroundType::Lumberyard)
                        .count()
                        >= 3
                    {
                        GroundType::Lumberyard
                    } else {
                        GroundType::Trees
                    }
                }
                GroundType::Lumberyard => {
                    let next_to_lumberyard =
                        World::adjacent(idx).any(|i| self.area[i] == GroundType::Lumberyard);
                    let next_to_trees =
                        World::adjacent(idx).any(|i| self.area[i] == GroundType::Trees);
                    if next_to_lumberyard && next_to_trees {
                        GroundType::Lumberyard
                    } else {
                        GroundType::Open
                    }
                }
            };
        }
        new_world
    }

    fn num_lumberyards(&self) -> usize {
        self.area
            .iter()
            .filter(|c| **c == GroundType::Lumberyard)
            .count()
    }

    fn num_wood(&self) -> usize {
        self.area
            .iter()
            .filter(|c| **c == GroundType::Trees)
            .count()
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day18/input").expect("Could not read input file");
    let mut lines = input.lines();
    let mut world = World::new();
    for y in 0..50 {
        let mut line = lines.next().unwrap().chars();
        for x in 0..50 {
            let idx = y * 50 + x;
            world.area[idx] = match line.next().unwrap() {
                '.' => GroundType::Open,
                '|' => GroundType::Trees,
                '#' => GroundType::Lumberyard,
                _ => unreachable!(),
            };
        }
    }

    {
        let mut world = world.clone();
        for _ in 0..10 {
            world = world.evolve();
        }

        println!("Result 1: {}", world.num_lumberyards() * world.num_wood());
    }

    {
        let target = 1_000_000_000;
        let mut hash_map = HashMap::new();
        hash_map.insert(world.clone(), 0);
        let mut minute = 0usize;
        while minute < target {
            let new_world = world.evolve();
            minute += 1;
            if let Some(repeat_minute) = hash_map.get(&new_world) {
                let loop_length = minute - repeat_minute;
                let loop_count = ((target - repeat_minute) / loop_length) as usize;
                let closest_minute = repeat_minute + loop_count * loop_length;
                minute = closest_minute;
            } else {
                hash_map.insert(new_world.clone(), minute);
            }
            world = new_world;
        }

        println!("Result 2: {}", world.num_lumberyards() * world.num_wood());
    }
}
