use std::io::Write;

pub fn clear_console() {
    // ANSI escape code to clear the console and reset the cursor position
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();
}
