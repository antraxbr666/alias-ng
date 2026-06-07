use base64::Engine;
use std::io::{self, Write};

pub fn copy_to_clipboard(text: &str) -> io::Result<()> {
    let encoded = base64::engine::general_purpose::STANDARD.encode(text);
    let sequence = format!("\x1b]52;c;{}\x07", encoded);
    io::stdout().write_all(sequence.as_bytes())?;
    io::stdout().flush()
}
