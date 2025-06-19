mod timer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let work = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(25);
    let short_break = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(5);
    let long_break = args.get(3).and_then(|a| a.parse().ok()).unwrap_or(15);

    timer::start_pomodoro(work, short_break, long_break);
}