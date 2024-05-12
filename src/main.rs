#![feature(let_chains)]

use owo_colors::OwoColorize;
use std::env::current_dir;

#[inline(always)]
fn home_dir() -> String {
    unsafe { std::env::var("HOME").unwrap_unchecked() }
}

#[inline(always)]
fn get_path() -> String {
    let path = unsafe { current_dir().unwrap_unchecked() };
    let path = path.to_string_lossy();
    let home = home_dir();
    if path.starts_with(&home) {
        if path == home {
            String::new()
        } else {
            format!("{} ", path.replace(&home, "~"))
        }
    } else {
        format!("{path} ")
    }
}

#[inline(always)]
fn get_grompt() -> String {
    match grompt::format_status(grompt::options::get_options()) {
        Ok(x) => format!("{x} "),
        Err(_) => Default::default(),
    }
}

#[inline(always)]
fn get_emoji() -> char {
    '🦦'
}

#[inline(always)]
fn get() {
    let path = get_path();
    let grompt = get_grompt();
    let emoji = get_emoji();
    print!("{}{}{}", path.yellow().bold(), grompt, emoji);
}

fn main() {
    // let max = 100;
    // let mut total_time = 0.0f64;
    // let mut slowest = 0.0f64;
    // let mut fastest = f64::MAX;
    // for _ in 0..max {
    //     let time = std::time::Instant::now();
    //     get();
    //     let elapsed = time.elapsed().as_secs_f64();
    //     total_time += elapsed;
    //     slowest = slowest.max(elapsed);
    //     fastest = fastest.min(elapsed);
    // }
    //
    // let avg = total_time / max as f64;
    //
    // println!(
    //     "Avg time was {avg}s ({}th of a second)",
    //     (1.0 / avg) as usize
    // );
    // println!(
    //     "Slowest time was {slowest}s ({}th of a second)",
    //     (1.0 / slowest) as usize
    // );
    // println!(
    //     "Fastest time was {fastest}s ({}th of a second)",
    //     (1.0 / fastest) as usize
    // );'
    get();
}

// Avg time was 0.00138062763s
// Slowest time was 0.061608917s
// Fastest time was 0.000668868s
