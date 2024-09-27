
  /*********************************************/
 /*** PROBABLY ONLY FOR OUTPUT/FORMAT STUFF ***/
/*********************************************/

use core::fmt;
use std::io;

pub struct Escaper<W>
where
    W: io::Write,
{
    pub writer: W,
    pub error: io::Result<()>,
}

mod things {
    use std::io::Write;

    use super::Escaper;


    impl<W:Write> Escaper<W> {
        pub fn _llvm_things(&mut self, _llfile:&[u8]) {

        }
    }
}
mod elfthings {
    use std::io::Write;

    use super::Escaper;


    impl<W:Write> Escaper<W> {
        pub fn _elf_things(&mut self, _llfile:&[u8]) {

        }
    }
}


// #[allow(dead_code)]
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
        match (|| -> io::Result<()> {
            match c {
                '"' => write!(self.writer, "\\")?,
                _ => {}
            }

            write!(self.writer, "{}", c)
        })() {
            Err(e) => {
                self.error = Err(e);

                Err(fmt::Error)
            }
            Ok(()) => Ok(()),
        }
    }
}


