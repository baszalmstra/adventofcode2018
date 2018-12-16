#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Opcode {
    Addr = 0,
    Addi = 1,
    Mulr = 2,
    Muli = 3,
    Banr = 4,
    Bani = 5,
    Borr = 6,
    Bori = 7,
    Setr = 8,
    Seti = 9,
    Gtir = 10,
    Gtri = 11,
    Gtrr = 12,
    Eqir = 13,
    Eqri = 14,
    Eqrr = 15,
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            0 => Opcode::Addr,
            1 => Opcode::Addi,
            2 => Opcode::Mulr,
            3 => Opcode::Muli,
            4 => Opcode::Banr,
            5 => Opcode::Bani,
            6 => Opcode::Borr,
            7 => Opcode::Bori,
            8 => Opcode::Setr,
            9 => Opcode::Seti,
            10 => Opcode::Gtir,
            11 => Opcode::Gtri,
            12 => Opcode::Gtrr,
            13 => Opcode::Eqir,
            14 => Opcode::Eqri,
            15 => Opcode::Eqrr,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct OpcodeArgs(u8, u8, u8);

type Registers = [i32; 4];

impl Opcode {
    pub fn apply(self, args: OpcodeArgs, registers: Registers) -> Registers {
        let mut result = registers;
        match self {
            Opcode::Addr => {
                result[args.2 as usize] = result[args.0 as usize] + result[args.1 as usize]
            }
            Opcode::Addi => result[args.2 as usize] = result[args.0 as usize] + args.1 as i32,
            Opcode::Mulr => {
                result[args.2 as usize] = result[args.0 as usize] * result[args.1 as usize]
            }
            Opcode::Muli => result[args.2 as usize] = result[args.0 as usize] * args.1 as i32,
            Opcode::Banr => {
                result[args.2 as usize] = result[args.0 as usize] & result[args.1 as usize]
            }
            Opcode::Bani => result[args.2 as usize] = result[args.0 as usize] & args.1 as i32,
            Opcode::Borr => {
                result[args.2 as usize] = result[args.0 as usize] | result[args.1 as usize]
            }
            Opcode::Bori => result[args.2 as usize] = result[args.0 as usize] | args.1 as i32,
            Opcode::Setr => result[args.2 as usize] = result[args.0 as usize],
            Opcode::Seti => result[args.2 as usize] = args.0 as i32,
            Opcode::Gtir => {
                result[args.2 as usize] = if args.0 as i32 > result[args.1 as usize] {
                    1
                } else {
                    0
                }
            }
            Opcode::Gtri => {
                result[args.2 as usize] = if result[args.0 as usize] > args.1 as i32 {
                    1
                } else {
                    0
                }
            }
            Opcode::Gtrr => {
                result[args.2 as usize] = if result[args.0 as usize] > result[args.1 as usize] {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqir => {
                result[args.2 as usize] = if args.0 as i32 == result[args.1 as usize] {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqri => {
                result[args.2 as usize] = if result[args.0 as usize] == args.1 as i32 {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqrr => {
                result[args.2 as usize] = if result[args.0 as usize] == result[args.1 as usize] {
                    1
                } else {
                    0
                }
            }
        };
        result
    }

    pub fn values() -> impl Iterator<Item = Opcode> {
        static VALUES: [Opcode; 16] = [
            Opcode::Addr,
            Opcode::Addi,
            Opcode::Mulr,
            Opcode::Muli,
            Opcode::Banr,
            Opcode::Bani,
            Opcode::Borr,
            Opcode::Bori,
            Opcode::Setr,
            Opcode::Seti,
            Opcode::Gtir,
            Opcode::Gtri,
            Opcode::Gtrr,
            Opcode::Eqir,
            Opcode::Eqri,
            Opcode::Eqrr,
        ];
        VALUES.iter().map(|c| *c)
    }
}

#[derive(Debug, Clone)]
struct Sample {
    before: Registers,
    opcode: u8,
    opcode_args: OpcodeArgs,
    after: Registers,
}

struct Execution(Opcode, OpcodeArgs);

fn main() {
    let input = std::fs::read_to_string("inputs/day16/input1").expect("Could not read input file");

    let samples = {
        let mut input_iter = input.lines();
        let mut samples = Vec::new();
        while let Some(before_line) = input_iter.next() {
            let mut before_registers = before_line[9..before_line.len() - 1]
                .split(", ")
                .map(|d| d.parse::<i32>().unwrap());
            let mut opcode = input_iter
                .next()
                .unwrap()
                .split(' ')
                .map(|d| d.parse::<u8>().unwrap());
            let after_line = input_iter.next().unwrap();
            let mut after_registers = after_line[9..after_line.len() - 1]
                .split(", ")
                .map(|d| d.parse::<i32>().unwrap());
            samples.push(Sample {
                before: [
                    before_registers.next().unwrap(),
                    before_registers.next().unwrap(),
                    before_registers.next().unwrap(),
                    before_registers.next().unwrap(),
                ],
                opcode: opcode.next().unwrap(),
                opcode_args: OpcodeArgs(
                    opcode.next().unwrap(),
                    opcode.next().unwrap(),
                    opcode.next().unwrap(),
                ),
                after: [
                    after_registers.next().unwrap(),
                    after_registers.next().unwrap(),
                    after_registers.next().unwrap(),
                    after_registers.next().unwrap(),
                ],
            });
            input_iter.next();
        }
        samples
    };

    let mut opcode_possibilities: [[bool; 16]; 16] = [[true; 16]; 16];
    let mut three_or_more = 0;
    for sample in samples.iter() {
        let mut matching_samples = 0;
        for opcode in Opcode::values() {
            let matches = opcode.apply(sample.opcode_args.clone(), sample.before) == sample.after;
            if matches {
                matching_samples += 1;
            } else {
                opcode_possibilities[sample.opcode as usize][opcode as usize] = false;
            }
        }
        if matching_samples >= 3 {
            three_or_more += 1;
        }
    }

    println!("Result 1: {}/{}", three_or_more, samples.len());

    // Figure out the opcode mappings by power of elimination
    let mut opcode_mapping: [Opcode; 16] = [Opcode::Addi; 16];
    for _ in 0..16 {
        for i in 0..16 {
            let options: Vec<usize> = opcode_possibilities[i]
                .iter()
                .enumerate()
                .filter(|(_, possible)| **possible)
                .map(|(i, _)| i)
                .collect();
            if options.len() == 1 {
                let opcode = Opcode::from(options[0] as u8);
                opcode_mapping[i] = opcode;
                for j in 0..16 {
                    opcode_possibilities[j][options[0]] = false;
                }
            }
        }
    }

    // Parse sample program
    let input = std::fs::read_to_string("inputs/day16/input2").expect("Could not read input file");
    let execution: Vec<Execution> = input
        .lines()
        .map(|l| {
            let mut digits = l.split(' ').map(|d| d.parse::<u8>().unwrap());
            Execution(
                opcode_mapping[digits.next().unwrap() as usize],
                OpcodeArgs(
                    digits.next().unwrap(),
                    digits.next().unwrap(),
                    digits.next().unwrap(),
                ),
            )
        })
        .collect();

    let mut registers = [0; 4];
    for exec in execution {
        registers = exec.0.apply(exec.1, registers);
    }

    println!("Result 2: {:?}", registers);
}
