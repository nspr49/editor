use core::fmt;
use std::ptr::write;

pub struct Buffer {
    pub rows: Vec<String>,
}

impl Buffer {
    pub fn empty_buffer() -> Self {
        Buffer { rows: Vec::new() }
    }

    pub fn new(rows: Vec<String>) -> Self {
        Buffer { rows }
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--------------Buffer--------------")?;
        for row in self.rows.iter() {
            writeln!(f, "{}", row)?;
        }
        writeln!(f, "")
    }
}
