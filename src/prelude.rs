
#![allow(warnings)] // @NOCHECKIN


      /*********************************************/
     /*********************************************/
    /*********************************************/
   /***         ACTUAL PRELUDE STUFF          ***/
  /*********************************************/
 /*********************************************/
/*********************************************/

use std::borrow::Cow;

  /*********************************************/
 /***             NODE                      ***/
/*********************************************/

#[derive(Clone)]
pub struct Node<'a> {
    pub name: Cow<'a, str>,
    pub local: Local,
    pub max: Option<Max>,
    pub dashed: bool,
}

#[allow(non_snake_case)]
pub fn Node<'a, S>(name: S, stack: Option<u64>, dashed: bool) -> Node<'a>
where
    S: Into<Cow<'a, str>>,
{
    Node {
        name: name.into(),
        local: stack.map(Local::Exact).unwrap_or(Local::Unknown),
        max: None,
        dashed,
    }
}





  /*********************************************/
 /***             LOCAL                     ***/
/*********************************************/
/// Local stack usage
#[derive(Clone, Copy, PartialEq)]
pub enum Local {
    Exact(u64),
    Unknown,
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Local::Exact(n) => write!(f, "{}", n),
            Local::Unknown => f.write_str("?"),
        }
    }
}

impl Into<Max> for Local {
    fn into(self) -> Max {
        match self {
            Local::Exact(n) => Max::Exact(n),
            Local::Unknown => Max::LowerBound(0),
        }
    }
}


  /*********************************************/
 /***             MAX                       ***/
/*********************************************/
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Max {
    Exact(u64),
    LowerBound(u64),
}

use core::{ops, cmp};

impl ops::Add<Local> for Max {
    type Output = Max;

    fn add(self, rhs: Local) -> Max {
        match (self, rhs) {
            (Max::Exact(lhs), Local::Exact(rhs)) => Max::Exact(lhs + rhs),
            (Max::Exact(lhs), Local::Unknown) => Max::LowerBound(lhs),
            (Max::LowerBound(lhs), Local::Exact(rhs)) => Max::LowerBound(lhs + rhs),
            (Max::LowerBound(lhs), Local::Unknown) => Max::LowerBound(lhs),
        }
    }
}

impl ops::Add<Max> for Max {
    type Output = Max;

    fn add(self, rhs: Max) -> Max {
        match (self, rhs) {
            (Max::Exact(lhs), Max::Exact(rhs)) => Max::Exact(lhs + rhs),
            (Max::Exact(lhs), Max::LowerBound(rhs)) => Max::LowerBound(lhs + rhs),
            (Max::LowerBound(lhs), Max::Exact(rhs)) => Max::LowerBound(lhs + rhs),
            (Max::LowerBound(lhs), Max::LowerBound(rhs)) => Max::LowerBound(lhs + rhs),
        }
    }
}

pub fn max_of(mut iter: impl Iterator<Item = Max>) -> Option<Max> {
    iter.next().map(|first| iter.fold(first, max))
}

pub fn max(lhs: Max, rhs: Max) -> Max {
    match (lhs, rhs) {
        (Max::Exact(lhs), Max::Exact(rhs)) => Max::Exact(cmp::max(lhs, rhs)),
        (Max::Exact(lhs), Max::LowerBound(rhs)) => Max::LowerBound(cmp::max(lhs, rhs)),
        (Max::LowerBound(lhs), Max::Exact(rhs)) => Max::LowerBound(cmp::max(lhs, rhs)),
        (Max::LowerBound(lhs), Max::LowerBound(rhs)) => Max::LowerBound(cmp::max(lhs, rhs)),
    }
}

impl fmt::Display for Max {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Max::Exact(n) => write!(f, "= {}", n),
            Max::LowerBound(n) => write!(f, ">= {}", n),
        }
    }
}



  /*********************************************/
 /*** PROBABLY ONLY FOR OUTPUT/FORMAT STUFF ***/
/*********************************************/

use core::fmt::{self, Write as _};
use std::io::{self, BufRead, BufReader, Read, Write};

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






