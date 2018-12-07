extern crate regex;

const FABRIC_SIZE: usize = 1000;

struct Claim {
    pub id: usize,
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

fn is_unique(claim: &Claim, fabric: &[u32; FABRIC_SIZE * FABRIC_SIZE]) -> bool {
    for y in claim.top..claim.top + claim.height {
        for x in claim.left..claim.left + claim.width {
            if fabric[y * FABRIC_SIZE + x] > 1 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let input = std::fs::read_to_string("inputs/day3/input").expect("Could not read puzzle input");

    // Parse the input into claims
    let re = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let claims: Vec<Claim> = input
        .lines()
        .map(|l| {
            let captures = re.captures(l).expect("Invalid claim");
            Claim {
                id: captures[1].parse().unwrap(),
                left: captures[2].parse().unwrap(),
                top: captures[3].parse().unwrap(),
                width: captures[4].parse().unwrap(),
                height: captures[5].parse().unwrap(),
            }
        })
        .collect();

    // Sum the inches that are used
    let mut fabric = [0_u32; FABRIC_SIZE * FABRIC_SIZE];
    for claim in claims.iter() {
        for y in claim.top..claim.top + claim.height {
            for x in claim.left..claim.left + claim.width {
                fabric[y * FABRIC_SIZE + x] += 1;
            }
        }
    }

    // Count the number of inches that have more than 1 claim
    let multiple_claim_count = fabric.iter().filter(|s| **s > 1).count();

    println!("Result 1: {}", multiple_claim_count);

    // Find the claim location were all entries are 1
    for claim in claims.iter() {
        if is_unique(claim, &fabric) {
            println!("Result 2: {}", claim.id)
        }
    }
}
