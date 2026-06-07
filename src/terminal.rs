use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

pub fn inject_to_tty(text: &str) {
    let tty = match OpenOptions::new().write(true).open("/dev/tty") {
        Ok(f) => f,
        Err(_) => return,
    };
    let fd = tty.as_raw_fd();
    for byte in text.bytes() {
        unsafe {
            libc::ioctl(fd, libc::TIOCSTI, &byte);
        }
    }
}
