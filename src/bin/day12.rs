use std::collections::VecDeque;

#[derive(Clone, Copy, Eq, PartialEq)]
enum PotState {
    Plant,
    Empty
}

impl From<char> for PotState {
    fn from(c: char) -> Self {
        match c {
            '#' => PotState::Plant,
            '.' => PotState::Empty,
            _ => unreachable!()
        }
    }
}

impl std::fmt::Display for PotState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            PotState::Plant => write!(f, "#"),
            PotState::Empty => write!(f, "."),
        }
    }
}

struct Rule {
    pattern: [PotState; 5],
    result: PotState
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}{}{}{} => {}",
            self.pattern[0],
            self.pattern[1],
            self.pattern[2],
            self.pattern[3],
            self.pattern[4],
            self.result
        )
    }
}

struct State {
    state: VecDeque<PotState>,
    offset: i64
}

impl State {
    fn new(initial_state:VecDeque<PotState>) -> State {
        let mut state = initial_state;
        let mut offset = 0;
        while let Some(PotState::Empty) = state.front() {
            state.pop_front();
            offset += 1;
        }

        while let Some(PotState::Empty) = state.back() {
            state.pop_back();
        }
        State { state, offset }
    }

    fn iterate(&self, rules:&Vec<Rule>) -> State {
        let mut previous_state = VecDeque::with_capacity(self.state.len() + 8);
        for _ in 0..4 {
            previous_state.push_back(PotState::Empty);
        }
        for v in self.state.iter() {
            previous_state.push_back(*v);
        }
        for _ in 0..4 {
            previous_state.push_back(PotState::Empty);
        }

        // Start matching
        let mut state = VecDeque::new();
        state.resize(previous_state.len(), PotState::Empty);
        for i in 2 .. previous_state.len() - 2 {
            for rule in rules.iter() {
                if previous_state[i - 2] == rule.pattern[0] &&
                    previous_state[i - 1] == rule.pattern[1] &&
                    previous_state[i - 0] == rule.pattern[2] &&
                    previous_state[i + 1] == rule.pattern[3] &&
                    previous_state[i + 2] == rule.pattern[4] {
                    state[i] = rule.result.clone();
                    break;
                }
            }
        }

        let mut offset = self.offset - 4;
        while let Some(PotState::Empty) = state.front() {
            state.pop_front();
            offset += 1;
        }

        while let Some(PotState::Empty) = state.back() {
            state.pop_back();
        }

        State {
            state,
            offset
        }
    }

    fn plant_sum(&self) -> i64 {
        self.state
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == PotState::Plant)
            .fold(0, |s, (i, _)| s + (i as i64 + self.offset))
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for v in self.state.iter() {
            write!(f, "{}", v)?
        }
        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day12/input")
        .expect("Could not read input file");

    let (initial_state, rules) = {
        let mut lines_iter = input.lines();
        let initial_state = State::new(
            (&(lines_iter.next().unwrap()[15..]))
                .chars()
                .map(|c| match c {
                    '#' => PotState::Plant,
                    _ => PotState::Empty
                })
                .collect());

        lines_iter.next(); // Skip white line

        let rules: Vec<Rule> = lines_iter.map(|line| {
            let chars: Vec<char> = line.chars().collect();
            Rule {
                pattern: [chars[0].into(),
                    chars[1].into(),
                    chars[2].into(),
                    chars[3].into(),
                    chars[4].into()],
                result: chars[9].into()
            }
        })
        .collect();
        (initial_state, rules)
    };

    let mut state = initial_state;
    //println!("0: {} (offset: {})", state, state.offset);
    for _i in 0..20 {
        let new_state = state.iterate(&rules);
        state = new_state;
    }
    let result1 = state.plant_sum();

    for _i in 20..2000 {
        let new_state = state.iterate(&rules);
        //println!("{}: {} (offset: {})", i+1, new_state, new_state.offset);
        state = new_state;
    }

    let guess = State {
        offset: state.offset + (50000000000-2000),
        state: state.state
    };

    println!("Result 1: {}", result1);
    println!("Result 2: {}", guess.plant_sum()); // Guessed sum after 50000000000 iterations
}