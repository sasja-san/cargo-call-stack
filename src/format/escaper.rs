
use core::fmt;
use std::io;

pub struct Escaper<W>
where
    W: io::Write,
{
    pub writer: W,
    pub error: io::Result<()>,
}


impl<W> Escaper<W>
where
    W: io::Write,
{
    pub fn new(writer: W) -> Self {
        Escaper {
            writer,
            error: Ok(()),
        }
    }
}

impl<W> fmt::Write for Escaper<W>
where
    W: io::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        if c == '"' {
            /* ████████╗██╗  ██╗███████╗    ██╗     ██╗███╗   ██╗███████╗    */
            /* ╚══██╔══╝██║  ██║██╔════╝    ██║     ██║████╗  ██║██╔════╝██╗ */
            /*    ██║   ███████║█████╗      ██║     ██║██╔██╗ ██║█████╗  ╚═╝ */
            /*    ██║   ██╔══██║██╔══╝      ██║     ██║██║╚██╗██║██╔══╝  ██╗ */
            /*    ██║   ██║  ██║███████╗    ███████╗██║██║ ╚████║███████╗╚═╝ */
            /*    ╚═╝   ╚═╝  ╚═╝╚══════╝    ╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝    */
            self.error = write!(self.writer, "\\");
            if let Err(_) = self.error {
                return Err(fmt::Error);
            }
        }

        self.error = write!(self.writer, "{}", c);
        if let Err(_) = self.error {
            return Err(fmt::Error); 
        }

        return Ok(());
    }
}


