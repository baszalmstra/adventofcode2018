extern crate chrono;

use chrono::offset::TimeZone;
use std::collections::HashMap;
use std::ops::Range;
use chrono::Timelike;

enum GuardActivity {
    WakesUp,
    FallsAsleep,
    BeginsShift(usize)
}

enum GuardState {
    Awake,
    Asleep(u32)
}

fn build_sleep_histogram(ranges:&Vec<Range<u32>>) -> [u32;60] {
    let mut histogram = [0;60];
    for range in ranges.iter() {
        for minute in range.start .. range.end {
            histogram[minute as usize] += 1;
        }
    }
    histogram
}

fn main() {
    let input = std::fs::read_to_string("inputs/day4/input")
        .expect("Could not read input file");

    // Get the input sort and in a machine readable format.
    let re = regex::Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)] ((falls asleep)|(wakes up)|(Guard #(\d+) begins shift))").unwrap();
    let events = {
        let mut events:Vec<(chrono::DateTime<chrono::Utc>, GuardActivity)> = input
            .lines()
            .map(|l| {
                let capture = re.captures(l).expect("Regex doesn't match");
                (chrono::Utc.ymd(
                    capture[1].parse().unwrap(),
                    capture[2].parse().unwrap(),
                    capture[3].parse().unwrap())
                     .and_hms(
                         capture[4].parse().unwrap(),
                         capture[5].parse().unwrap(),
                         0
                     ), match &capture[6] {
                    "wakes up" => GuardActivity::WakesUp,
                    "falls asleep" => GuardActivity::FallsAsleep,
                    _ => GuardActivity::BeginsShift(capture[10].parse().unwrap())

                })
            })
            .collect();

        // Sort the events by date
        events.sort_by(|(a,_),(b, _)| a.cmp(b) );
        events
    };

    // Convert the input into a map of sleep ranges
    let sleep_ranges = {
        let mut sleep_ranges: HashMap<usize, Vec<Range<u32>>> = HashMap::new();
        let mut current_guard_id = 0;
        let mut current_guard_state = GuardState::Awake;
        for event in events.iter() {
            match event.1 {
                GuardActivity::BeginsShift(id) => {
                    current_guard_id = id;
                    current_guard_state = GuardState::Awake;
                },
                GuardActivity::FallsAsleep => {
                    current_guard_state = GuardState::Asleep(event.0.time().minute());
                },
                GuardActivity::WakesUp => {
                    if let GuardState::Asleep(time) = current_guard_state {
                        sleep_ranges.entry(current_guard_id)
                            .and_modify(|e| e.push(time..event.0.time().minute()))
                            .or_insert(vec![time..event.0.time().minute()]);
                    } else {
                        unreachable!();
                    }
                }
            }
        }
        sleep_ranges
    };

    let most_asleep_guard = sleep_ranges.iter()
        .map(|entry| (entry.0, entry.1.iter()
            .fold(0, |s, r| s + r.end - r.start )))
        .max_by(|(_, a),(_,b)| a.cmp(b))
        .unwrap()
        .0;

    let max_minute = sleep_ranges.get(most_asleep_guard)
        .map(build_sleep_histogram)
        .map(|histogram| histogram.iter()
            .enumerate()
            .max_by(|(_,a), (_,b)| a.cmp(b))
            .unwrap()
            .0)
        .unwrap();

    println!("Result 1: {}", max_minute*most_asleep_guard);

    let most_frequent_guard = sleep_ranges.iter()
        .map(|(id, range)| (id, build_sleep_histogram(range)))
        .map(|(id, histogram)| (id, histogram.iter()
            .enumerate()
            .max_by(|(_,a), (_,b)| a.cmp(b))
            .map(|(id, count)| (id, *count))
            .unwrap()))
        .max_by(|(_, (_, a)), (_, (_, b))| a.cmp(b))
        .map(|(id, (minute, _))| (id, minute))
        .unwrap();

    println!("Result 2: {}", *most_frequent_guard.0*most_frequent_guard.1);
}