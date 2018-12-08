fn sum_meta_entries<'a, T>(it: &mut T) -> u32
where
    T: Iterator<Item = &'a u32>,
{
    let child_count = it.next().unwrap();
    let meta_entries_count = it.next().unwrap();

    let mut meta_entries_sum = 0;
    for _ in 0..*child_count {
        meta_entries_sum += sum_meta_entries(it);
    }

    for _ in 0..*meta_entries_count {
        meta_entries_sum += it.next().unwrap();
    }

    meta_entries_sum
}

fn compute_value<'a, T>(it: &mut T) -> u32
where
    T: Iterator<Item = &'a u32>,
{
    let child_count = it.next().unwrap();
    let meta_entries_count = it.next().unwrap();

    if *child_count == 0 {
        // If there are no simply sum the meta entries together
        let mut meta_entries_sum = 0;
        for _ in 0..*meta_entries_count {
            meta_entries_sum += it.next().unwrap();
        }
        meta_entries_sum
    } else {
        // If there are children, compute their root value
        let mut child_values = Vec::with_capacity(*child_count as usize);
        for _ in 0..*child_count {
            child_values.push(compute_value(it));
        }

        // Use the meta entries as references
        let mut meta_entries_sum = 0;
        for _ in 0..*meta_entries_count {
            let meta_entry = it.next().unwrap();
            if *meta_entry > 0 && *meta_entry as usize <= child_values.len() {
                meta_entries_sum += child_values[(meta_entry - 1) as usize];
            }
        }

        meta_entries_sum
    }
}

fn main() {
    let input: Vec<u32> = std::fs::read_to_string("inputs/day8/input")
        .expect("Could not read input file")
        .split(' ')
        .map(|d| d.parse().unwrap())
        .collect();

    let meta_entries_sum = sum_meta_entries(&mut input.iter());
    let root_value = compute_value(&mut input.iter());

    println!("Result 1: {}", meta_entries_sum);
    println!("Result 2: {}", root_value);
}
