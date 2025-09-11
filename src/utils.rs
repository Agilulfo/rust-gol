use std::thread;
use std::time::Duration;

// TODO: is it possible to use an escape sequence that works also on emacs window?
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn wait(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}
