use std::{env, io::{Result, Write, stdin, stdout}, thread, time::{Duration, Instant}};

use autopilot::key::{Code, KeyCode};

const DEFAULT_SEC_INTERVAL: u64 = 30;

fn read_interval_input() -> Result<u64> {
    print!("Input awake interval in seconds (Empty will be default as 30 secs): ");
    stdout().flush()?;
    
    let mut inp = String::new();
    stdin().read_line(&mut inp)?;
    Ok(inp.trim_end().parse::<u64>().unwrap_or(DEFAULT_SEC_INTERVAL))
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let duration_str = args.get(1);
    let dur = if let Some(dur) = duration_str {
        dur.parse::<u64>().unwrap()
    } else {
        read_interval_input().unwrap()
    };

    println!("Keep awake start for every {} secs", dur);
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
