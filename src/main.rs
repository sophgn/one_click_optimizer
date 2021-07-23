use std::{thread, time};
use indicatif::ProgressBar;

fn main() {
    println!("Optimizing your computer, please wait...");
    let bar = ProgressBar::new(100);
    let duration = time::Duration::from_millis(50);
    for _ in 0..100 {
        bar.inc(1);
        thread::sleep(duration);
    }
    bar.finish();
    println!("\nCongratulation, your system is faster now!");
}
