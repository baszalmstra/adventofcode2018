use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::rc::Rc;

struct Step {
    char: String,
    followup_steps: Vec<Rc<RefCell<Step>>>,
    before_step_count: usize,
}

impl Step {
    fn new(char: String) -> Step {
        Step {
            char,
            followup_steps: Vec::new(),
            before_step_count: 0,
        }
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        other.char == self.char
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.char.partial_cmp(&self.char)
    }
}

impl Eq for Step {}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.char.cmp(&self.char)
    }
}

type StepQueue = BinaryHeap<Rc<RefCell<Step>>>;

fn build_queue(input: &str) -> StepQueue {
    // Parse the graph
    let steps: Vec<Rc<RefCell<Step>>> = {
        let re =
            regex::Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin\.")
                .unwrap();
        let mut steps = HashMap::new();
        for capture in input.lines().map(|l| re.captures(l).unwrap()) {
            let a = capture[1].to_owned();
            let b = capture[2].to_owned();
            let step_b = steps
                .entry(b.clone())
                .or_insert_with(|| Rc::new(RefCell::new(Step::new(b.clone()))))
                .clone();
            let step_a = steps
                .entry(a.clone())
                .or_insert_with(|| Rc::new(RefCell::new(Step::new(a.clone()))));
            step_b.borrow_mut().before_step_count += 1;
            step_a.borrow_mut().followup_steps.push(step_b);
        }
        steps.into_iter().map(|(_, v)| v).collect()
    };

    // Build up the queue by finding all root steps
    BinaryHeap::from_iter(steps.iter().filter_map(|f| {
        if f.borrow().before_step_count == 0 {
            Some(f.clone())
        } else {
            None
        }
    }))
}

fn work_done(step: &Rc<RefCell<Step>>, queue: &mut StepQueue) {
    let step = step.borrow();

    // Update all next steps
    for next_step in step.followup_steps.iter() {
        {
            let mut next_step_ref = next_step.borrow_mut();
            next_step_ref.before_step_count -= 1;
            if next_step_ref.before_step_count != 0 {
                continue;
            }
        }
        queue.push(next_step.clone());
    }
}

fn part1(mut queue: StepQueue) {
    print!("Result 1: ");

    // Now iterate over the queue (basically bread first search)
    while let Some(step) = queue.pop() {
        print!("{}", step.borrow().char);
        work_done(&step, &mut queue);
    }

    println!();
}

#[derive(Clone)]
enum Worker {
    Idle,
    Working(Rc<RefCell<Step>>, u32),
}

impl Worker {
    fn put_to_work(step: Rc<RefCell<Step>>) -> Worker {
        let time = 60 + step.borrow().char.as_bytes()[0] - 64;
        Worker::Working(step, u32::from(time))
    }

    fn is_idle(&self) -> bool {
        match self {
            Worker::Idle => true,
            _ => false,
        }
    }
}

fn part2(mut queue: StepQueue) {
    let mut workers = [
        Worker::Idle,
        Worker::Idle,
        Worker::Idle,
        Worker::Idle,
        Worker::Idle,
    ];
    let mut total_time = 0;

    print!("Result 2: ");

    // As long as there are steps to complete ..
    while !queue.is_empty() || !workers.iter().all(|w| w.is_idle()) {
        // Try to assign all idle workers
        for worker in workers.iter_mut().filter(|w| w.is_idle()) {
            if let Some(work) = queue.pop() {
                *worker = Worker::put_to_work(work);
            } else {
                break;
            }
        }

        // Skip time until a worker is done
        let skip_time = workers
            .iter()
            .filter_map(|w| match w {
                Worker::Idle => None,
                Worker::Working(_, b) => Some(*b),
            })
            .min()
            .unwrap();

        for worker in workers.iter_mut() {
            if let Worker::Working(work, time) = worker {
                *time -= skip_time;
                if *time == 0 {
                    print!("{}", work.borrow().char);
                    work_done(work, &mut queue);
                    *worker = Worker::Idle;
                }
            }
        }

        total_time += skip_time;
    }

    println!(", took: {}", total_time);
}

fn main() {
    let input = std::fs::read_to_string("inputs/day7/input").expect("Could not read input file");

    part1(build_queue(&input));
    part2(build_queue(&input));
}
