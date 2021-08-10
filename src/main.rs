use std::{
    thread,
    time::{Duration, Instant},
    env
};

use autopilot::key::{Code, KeyCode};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let duration_str = args.get(1);
    let dur = if let Some(dur) = duration_str {
        dur.parse::<u64>().unwrap()
    } else {
        30
    };
    let start_time = Instant::now();
    loop {
        thread::sleep(Duration::from_secs(dur));
        autopilot::key::tap(&Code(KeyCode::Control), &[], 100, 0);
        println!(
            "Nudges {}",
            Instant::now().duration_since(start_time).as_secs()
        );
    }
}
