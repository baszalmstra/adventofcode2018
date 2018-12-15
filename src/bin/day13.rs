enum Track {
    Horizontal,
    Vertical,
    Crossing,
    CurveRight,
    CurveLeft,
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum TurnDirection {
    Left = 0,
    Straight = 1,
    Right = 2,
}

#[derive(Clone)]
struct Cart {
    orientation: Orientation,
    turn_direction: TurnDirection,
    position: (usize, usize),
}

impl Cart {
    fn new(orientation: Orientation, position: (usize, usize)) -> Cart {
        Cart {
            orientation,
            turn_direction: TurnDirection::Left,
            position,
        }
    }

    fn update_position(&mut self) {
        let (x, y) = self.position;
        self.position = match self.orientation {
            Orientation::Up => (x, y - 1),
            Orientation::Down => (x, y + 1),
            Orientation::Left => (x - 1, y),
            Orientation::Right => (x + 1, y),
        };
    }

    fn update_orientation(&mut self, track: &Track) {
        match track {
            Track::CurveLeft => match self.orientation {
                Orientation::Up => self.orientation = Orientation::Left,
                Orientation::Right => self.orientation = Orientation::Down,
                Orientation::Left => self.orientation = Orientation::Up,
                Orientation::Down => self.orientation = Orientation::Right,
            },
            Track::CurveRight => match self.orientation {
                Orientation::Up => self.orientation = Orientation::Right,
                Orientation::Right => self.orientation = Orientation::Up,
                Orientation::Left => self.orientation = Orientation::Down,
                Orientation::Down => self.orientation = Orientation::Left,
            },
            Track::Crossing => {
                self.orientation = match self.orientation {
                    Orientation::Up => match self.turn_direction {
                        TurnDirection::Left => Orientation::Left,
                        TurnDirection::Straight => Orientation::Up,
                        TurnDirection::Right => Orientation::Right,
                    },
                    Orientation::Down => match self.turn_direction {
                        TurnDirection::Left => Orientation::Right,
                        TurnDirection::Straight => Orientation::Down,
                        TurnDirection::Right => Orientation::Left,
                    },
                    Orientation::Left => match self.turn_direction {
                        TurnDirection::Left => Orientation::Down,
                        TurnDirection::Straight => Orientation::Left,
                        TurnDirection::Right => Orientation::Up,
                    },
                    Orientation::Right => match self.turn_direction {
                        TurnDirection::Left => Orientation::Up,
                        TurnDirection::Straight => Orientation::Right,
                        TurnDirection::Right => Orientation::Down,
                    },
                };
                self.turn_direction = match self.turn_direction {
                    TurnDirection::Left => TurnDirection::Straight,
                    TurnDirection::Straight => TurnDirection::Right,
                    TurnDirection::Right => TurnDirection::Left,
                };
            }
            _ => {}
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day13/input").expect("Could not read input file");

    let world: Vec<Vec<Option<Track>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '/' => Some(Track::CurveRight),
                    '|' => Some(Track::Vertical),
                    '\\' => Some(Track::CurveLeft),
                    '-' => Some(Track::Horizontal),
                    '+' => Some(Track::Crossing),
                    '^' => Some(Track::Vertical),
                    '>' => Some(Track::Horizontal),
                    'v' => Some(Track::Vertical),
                    '<' => Some(Track::Horizontal),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let mut carts: Vec<Cart> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '>' => Some(Cart::new(Orientation::Right, (x, y))),
                '<' => Some(Cart::new(Orientation::Left, (x, y))),
                '^' => Some(Cart::new(Orientation::Up, (x, y))),
                'v' => Some(Cart::new(Orientation::Down, (x, y))),
                _ => None,
            })
        })
        .flatten()
        .collect();

    let mut iteration = 1;
    loop {
        // Sort the carts
        carts.sort_by(|a, b| {
            let (ax, ay) = a.position;
            let (bx, by) = b.position;
            if ay < by {
                std::cmp::Ordering::Less
            } else if ay > by {
                std::cmp::Ordering::Greater
            } else {
                ax.cmp(&bx)
            }
        });

        // Update all carts
        let mut colliding_carts: Option<(usize, usize)> = None;
        for i in 0..carts.len() {
            carts[i].update_position();

            for j in 0..carts.len() {
                if i != j {
                    if carts[i].position == carts[j].position {
                        println!(
                            "Found collision at {},{} iteration: {}",
                            carts[i].position.0, carts[i].position.1, iteration
                        );
                        colliding_carts = Some((i, j));
                    }
                }
            }

            let (x, y) = carts[i].position;
            carts[i].update_orientation(world[y][x].as_ref().unwrap());
        }

        // Remove colliding carts
        if let Some((cart_a, cart_b)) = colliding_carts {
            let min_index = cart_a.min(cart_b);
            let max_index = cart_a.max(cart_b);
            carts.remove(min_index);
            carts.remove(max_index - 1);
        }

        if carts.len() == 1 {
            let (x, y) = carts[0].position;
            println!("Last cart is at: {},{}", x, y);
            return;
        }

        iteration += 1;
    }
}
