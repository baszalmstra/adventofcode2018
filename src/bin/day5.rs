fn react_full<'a, T:Iterator<Item=&'a u8>>(polymer:T) -> Vec<u8> {
    let mut value:Vec<u8> = polymer.map(|a| *a).collect();
    let mut i = 0_usize;
    loop {
        if i == value.len() - 1 {
            return value;
        }

        if value[i] + 32 == value[i+1] ||
            value[i] - 32 == value[i+1] {
            value.remove(i);
            value.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day5/input")
        .expect("Could not read input file");

    let polymer = react_full(input.as_bytes().iter());

    println!("Result 1: {}", polymer.len());

    let value = (97_u8..123_u8)
        .map(|idx| {
            react_full(input.as_bytes().iter().filter(|p| {
               **p != idx && **p != idx - 32
            })).len()
        })
        .min()
        .unwrap();

    println!("Result 2: {:?}", value);
}