use std::collections::VecDeque;

/// Circle implements a circular vector with the help of a `VecDeque`. Insertion and removal is fast
/// at the beginning of the `VecDeque` so the cursor is always kept at the beginning of the
/// `VecDeque`.
struct Circle {
    vec: VecDeque<usize>,
}

impl Circle {
    pub fn new(last_marble_value: usize) -> Circle {
        let mut vec = VecDeque::with_capacity(last_marble_value + 1);
        vec.push_back(0);
        Circle { vec }
    }

    pub fn move_counter_clockwise(&mut self, count: usize) {
        for _ in 0..count {
            let val = self.vec.pop_back().unwrap();
            self.vec.push_front(val);
        }
    }

    pub fn move_clockwise(&mut self, count: usize) {
        for _ in 0..count {
            let val = self.vec.pop_front().unwrap();
            self.vec.push_back(val);
        }
    }

    pub fn insert(&mut self, value: usize) {
        self.vec.push_front(value);
    }

    pub fn remove(&mut self) -> Option<usize> {
        self.vec.pop_front()
    }
}

fn find_highest_score(player_count: usize, last_marble_value: usize) -> usize {
    let mut player_scores = Vec::with_capacity(player_count);
    player_scores.resize(player_count, 0);

    let mut circle = Circle::new(last_marble_value);

    for turn in 1..=last_marble_value {
        if turn % 23 == 0 {
            circle.move_counter_clockwise(7);
            let score = turn + circle.remove().unwrap();
            let player = (turn - 1) % player_scores.len();
            player_scores[player] += score;
        } else {
            circle.move_clockwise(2);
            circle.insert(turn);
        }
    }

    *player_scores.iter().max().unwrap()
}

fn main() {
    let input = &std::fs::read_to_string("inputs/day9/input").expect("Could not read input file");
    let regex = regex::Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let captures = regex.captures(input).expect("Invalid regex match");

    let player_count = captures[1].parse().unwrap();
    let last_marble_value = captures[2].parse().unwrap();

    let result1 = find_highest_score(player_count, last_marble_value);
    println!("Result 1: {}", result1);

    let result2 = find_highest_score(player_count, last_marble_value * 100);
    println!("Result 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_highest_score(9, 25), 32);
        assert_eq!(find_highest_score(10, 1618), 8317);
        assert_eq!(find_highest_score(13, 7999), 146373);
        assert_eq!(find_highest_score(17, 1104), 2764);
        assert_eq!(find_highest_score(21, 6111), 54718);
        assert_eq!(find_highest_score(30, 5807), 37305);
    }
}
