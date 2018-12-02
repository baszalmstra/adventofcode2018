use std::collections::HashSet;

fn main() {
    let input:Vec<i64> = std::fs::read_to_string("inputs/day1/input")
        .expect("Could not read input file")
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    // Sum the entries together
    let result = input.iter().fold(0, |a,b| a+b);
    println!("Result: {}", result);

    // Iterate over all entries (forever) and record which states we've come across.
    let mut values_seen= HashSet::<i64>::new();
    let mut current_frequency_index:usize = 0;
    let mut current_frequency = 0;
    values_seen.insert(current_frequency);
    loop {
        current_frequency += input[current_frequency_index];
        if let Some(_) = values_seen.get(&current_frequency) {
            println!("First double frequency: {}", current_frequency);
            return;
        }
        values_seen.insert(current_frequency);
        current_frequency_index = (current_frequency_index + 1) % input.len();
    }
}