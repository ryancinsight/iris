use std::io::{self, Write};

pub(super) struct Escaped<W>(pub(super) W);

impl<W: Write> Write for Escaped<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let mut start = 0;
        // HTML recognizes a closing script tag before JSON parsing. Escaping
        // '<' preserves the exact decoded string and keeps it inside the node.
        for (index, _) in bytes.iter().enumerate().filter(|(_, byte)| **byte == b'<') {
            self.0.write_all(&bytes[start..index])?;
            self.0.write_all(br"\u003c")?;
            start = index + 1;
        }
        self.0.write_all(&bytes[start..])?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
