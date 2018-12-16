use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Display;
use std::iter::FromIterator;

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Debug, Copy, Hash)]
struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn adjacent_positions(self) -> [Position; 4] {
        [
            Position::new(self.x, self.y - 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y + 1),
        ]
    }

    fn is_adjacent_to(self, other: Position) -> bool {
        (self.x - other.x).abs() + (self.y - other.y).abs() == 1
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Clone)]
struct Unit {
    unit: UnitType,
    health_points: i32,
    position: Position,
}

impl Unit {
    fn new(unit: UnitType, position: Position) -> Unit {
        Unit {
            unit,
            health_points: 200,
            position,
        }
    }

    fn is_alive(&self) -> bool {
        self.health_points > 0
    }

    fn is_dead(&self) -> bool {
        self.health_points <= 0
    }
}

#[derive(Clone)]
enum Tile {
    Wall,
    Empty,
    Unit(UnitType),
}

#[derive(Clone)]
struct World {
    tiles: Vec<Vec<Tile>>,
    units: Vec<Unit>,
}

impl World {
    fn set_tile(&mut self, position: Position, tile: Tile) {
        self.tiles[position.y as usize][position.x as usize] = tile;
    }

    fn is_open(&self, position: Position) -> bool {
        self.get_tile_at(position).map_or(false, |t| match t {
            Tile::Empty => true,
            _ => false,
        })
    }

    fn get_tile_at(&self, position: Position) -> Option<&Tile> {
        if position.x < 0 || position.y < 0 {
            None
        } else {
            self.tiles
                .get(position.y as usize)
                .and_then(|row| row.get(position.x as usize))
        }
    }

    fn find_closest_tile<'a, T>(&self, from: Position, to: T) -> Option<Position>
    where
        T: Iterator<Item = &'a Position>,
    {
        let targets: HashSet<Position> = HashSet::from_iter(to.map(|p| *p));

        if targets.contains(&from) {
            return None;
        }

        // Flood fill the tiles and record the first known position we come across
        let mut queue: VecDeque<Position> = VecDeque::new();
        queue.push_back(from);

        let mut shortest_paths = HashMap::<Position, Position>::new();
        while let Some(pos) = queue.pop_front() {
            if targets.contains(&pos) {
                let mut first_pos = pos;
                while let Some(parent_pos) = shortest_paths.get(&first_pos) {
                    if *parent_pos == from {
                        break;
                    }
                    first_pos = *parent_pos;
                }
                return Some(first_pos);
            }

            for new_pos in &pos.adjacent_positions() {
                if shortest_paths.contains_key(new_pos) || !self.is_open(*new_pos) {
                    continue;
                }
                queue.push_back(*new_pos);
                shortest_paths.insert(*new_pos, pos);
            }
        }

        None
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for line in self.tiles.iter() {
            for tile in line.iter() {
                match tile {
                    Tile::Wall => write!(f, "#")?,
                    Tile::Empty => write!(f, ".")?,
                    Tile::Unit(t) => match t {
                        UnitType::Elf => write!(f, "E")?,
                        UnitType::Goblin => write!(f, "G")?,
                    },
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct SimulationResult(UnitType, i32, i32);

fn run_simulation(mut world: World, elfs_attack_points: i32) -> SimulationResult {
    let mut round = 1;
    let mut elf_deaths = 0;
    loop {
        // Iterate over all units
        for i in 0..world.units.len() {
            let mut current_position = world.units[i].position;
            let unit_type = world.units[i].unit;

            // Skip units killed in this round
            if world.units[i].is_dead() {
                continue;
            }

            // Find the other life targets
            let mut targets = world
                .units
                .iter()
                .filter(|u| {
                    *u as *const Unit != &world.units[i] as *const Unit
                        && u.unit != world.units[i].unit
                        && u.is_alive()
                })
                .peekable();

            // See if there are still targets
            if targets.peek().is_none() {
                // The battle is over
                let healt_points_sum: i32 = world
                    .units
                    .iter()
                    .filter(|u| u.is_alive())
                    .map(|u| u.health_points)
                    .sum();
                return SimulationResult(unit_type, healt_points_sum * (round - 1), elf_deaths);
            }

            // If there are no targets in range
            if !targets
                .clone()
                .any(|target| target.position.is_adjacent_to(current_position))
            {
                // Find all adjacent squares
                let mut target_positions: Vec<Position> = targets
                    .map(|t| t.position.adjacent_positions().to_vec())
                    .flatten()
                    .filter(|pos| world.is_open(*pos))
                    .collect();
                target_positions.dedup();
                target_positions.sort();

                // Find the closest reachable square
                if let Some(move_square) =
                    world.find_closest_tile(world.units[i].position, target_positions.iter())
                {
                    let mut unit = &mut world.units[i];
                    let old_position = unit.position;
                    let unit_type = unit.unit;
                    unit.position = move_square;
                    world.set_tile(old_position, Tile::Empty);
                    world.set_tile(move_square, Tile::Unit(unit_type));
                    current_position = move_square;
                }
            }

            // Find the targets that are currently in range
            let attack_points = match unit_type {
                UnitType::Goblin => 3,
                UnitType::Elf => elfs_attack_points,
            };
            let mut targets: Vec<&mut Unit> = world
                .units
                .iter_mut()
                .filter(|u| {
                    u.is_alive()
                        && u.position.is_adjacent_to(current_position)
                        && u.unit != unit_type
                })
                .collect();
            let min_hitpoints = targets.iter().map(|u| u.health_points).min();
            targets.retain(|u| u.health_points == min_hitpoints.unwrap());
            targets.sort_by(|a, b| a.position.cmp(&b.position));
            if let Some(closest_target) = targets.first_mut() {
                let position = closest_target.position;
                closest_target.health_points -= attack_points;
                if closest_target.is_dead() {
                    world.set_tile(position, Tile::Empty);
                    if unit_type != UnitType::Elf {
                        elf_deaths += 1;
                    }
                }
            }
        }

        world.units.retain(|u| u.is_alive());
        world.units.sort_by(|a, b| a.position.cmp(&b.position));

        round += 1;
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day15/input").expect("Could not read input file");

    // Parse the input
    let mut world = World {
        tiles: Vec::new(),
        units: Vec::new(),
    };

    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            row.push(match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'E' => {
                    world
                        .units
                        .push(Unit::new(UnitType::Elf, Position::new(x, y)));
                    Tile::Unit(UnitType::Elf)
                }
                'G' => {
                    world
                        .units
                        .push(Unit::new(UnitType::Goblin, Position::new(x, y)));
                    Tile::Unit(UnitType::Goblin)
                }
                _ => unreachable!(),
            })
        }
        world.tiles.push(row);
    }

    let result1 = run_simulation(world.clone(), 3);
    println!("Result 1: {}", result1.1);

    let mut attack_power = 4;
    let mut result2 = run_simulation(world.clone(), attack_power);
    while result2.2 != 0 {
        attack_power += 1;
        result2 = run_simulation(world.clone(), attack_power);
    }
    println!("Result 2: {} (attack_power: {})", result2.1, attack_power);
}
