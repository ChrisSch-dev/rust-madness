use std::{thread, time::Duration};

pub fn start_pomodoro(work: u64, short_break: u64, long_break: u64) {
    let mut cycles = 0;
    loop {
        println!("Work for {} minutes!", work);
        countdown(work * 60);
        cycles += 1;
        if cycles % 4 == 0 {
            println!("Take a long break for {} minutes.", long_break);
            countdown(long_break * 60);
        } else {
            println!("Take a short break for {} minutes.", short_break);
            countdown(short_break * 60);
        }
    }
}

fn countdown(secs: u64) {
    for remaining in (1..=secs).rev() {
        print!("\r{} seconds left...", remaining);
        let _ = std::io::Write::flush(&mut std::io::stdout());
        thread::sleep(Duration::from_secs(1));
    }
    println!("\rTime's up!               ");
}