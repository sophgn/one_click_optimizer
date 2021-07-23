use std::io::{self, BufWriter, Write};
use std::{thread, time};
use terminal_size::{terminal_size, Width};

fn main() {
    let size = terminal_size();
    let mut terminal_width: usize = 80;
    if let Some((Width(w), _)) = size {
        terminal_width = w.into();
    }
    let progress_bar_length: usize = terminal_width - 2;
    println!("Optimizing your computer, please wait...");
    let duration = time::Duration::from_millis(50);
    let mut sw = BufWriter::new(io::stdout());
    
    for progess in 0..progress_bar_length {
        sw.write_fmt(format_args!(
            "\r[{}>{}]",
            "=".repeat(progress),
            " ".repeat(progress_bar_length - progress - 1)
        ));
        sw.flush();
        thread::sleep(duration);
    }
    
    sw.write_fmt(format_args!(
        "\r[{}]",
        "=".repeat(progress_bar_length)
    ));
    sw.flush();
    println!("\nCongratulation, your system is faster now!");
}
