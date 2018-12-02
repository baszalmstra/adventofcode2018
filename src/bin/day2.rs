fn main() {
    let input = std::fs::read_to_string("inputs/day2/input")
        .expect("Could not read puzzle input");

    let lines:Vec<&[u8]> = input
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    // Walk over all lines and construct an array that contains for every char from a..z
    // how many times it occurred.
    let counts:Vec<[usize;26]> = lines.iter()
        .map(|line| {
            let mut counts = [0;26];
            for c in line.iter() {
                counts[(c - 97) as usize] += 1;
            }
            counts
        })
        .collect();

    // Count the number of times at least one element appeared twice
    let double_count = counts.iter()
        .filter(|counts| counts.iter().any(|v| *v==2))
        .count();

    // Count the number of times at least one element appeared three times
    let triple_count = counts.iter()
        .filter(|counts| counts.iter().any(|v| *v==3))
        .count();

    println!("Part one: {}", double_count*triple_count);

    // Match all ids with all other ids
    for (idx, a) in lines.iter().enumerate() {
        for b in lines[idx+1 .. ].iter() {
            // Find the dissimilar characters
            if a.iter()
                .zip(b.iter())
                .filter(|(a,b)| a!=b)
                .count() == 1 {
                // Found the matching box ID's, now find the characters that do match
                let similar:Vec<u8> = a.iter()
                    .zip(b.iter())
                    .filter_map(|(a,b)| if a==b { Some(*a) } else { None } )
                    .collect();
                println!("Part two: {}", String::from_utf8(similar).unwrap());
            };
        }
    }
}