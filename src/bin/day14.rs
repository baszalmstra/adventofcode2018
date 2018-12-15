use std::collections::VecDeque;

fn digits(count:usize) -> Vec<usize> {
    let mut value = count;
    let mut result = Vec::new();
    loop {
        let digit = value % 10;
        result.push(digit);
        value = (value - digit)/10;

        if value == 0 {
            break;
        }
    }

    result.reverse();
    result
}

fn compare_end(scores:&VecDeque<usize>, request:&Vec<usize>) -> bool {
    let size = request.len();
    if scores.len() < size {
        return false;
    }

    let begin = scores.len() - size;
    for i in 0 .. request.len() {
        if scores[i+begin] != request[i] {
            return false;
        }
    }

    true
}

fn main() {
    let input = 110201;
    let input_digits = digits(input);

    let mut scoreboard = VecDeque::new();
    scoreboard.push_back(3usize);
    scoreboard.push_back(7usize);

    let mut elves = Vec::with_capacity(2);
    elves.push(0usize);
    elves.push(1usize);

    loop {
        // Compute sum of current recipes
        let sum = elves.iter().fold(0usize, |state, current_recipe| state + scoreboard[*current_recipe]);

        // Append the digits to the scoreboard
        let digits = digits(sum);
        for digit in digits.into_iter() {
            scoreboard.push_back(digit);

            if scoreboard.len() == input + 10 {
                print!("Result 1: ");
                let begin = input;
                let end = input + 10;
                for i in begin .. end {
                    print!("{}", scoreboard[i])
                }
                println!();
            }

            if compare_end(&scoreboard, &input_digits ) {
                println!("Result 2: {}", scoreboard.len() - input_digits.len());
                return;
            }
        }

        // Update current recipes
        for current_recipe in elves.iter_mut() {
            *current_recipe = (*current_recipe + scoreboard[*current_recipe] + 1) % scoreboard.len();
        }
    }
}