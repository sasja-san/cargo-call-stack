use core::{fmt::Write};
use std::{io,io::Write as _};

use super::escaper::Escaper;
use crate::prelude::*;

use petgraph::Graph;

// use super::escaper;

pub fn top(g: &Graph<Node, ()>) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    assert!(g.is_directed());

    let mut nodes: Vec<Node> = Vec::new();
    for node in g.raw_nodes().iter() {
        nodes.push(node.weight.clone());
    }

    // Locate max
    if let Some(max) = max_of(nodes.iter().map(|n| n.max.unwrap_or(Max::Exact(0)))) {
        writeln!(
            stdout,
            "{} MAX",
            match max {
                Max::Exact(n) => n,
                Max::LowerBound(n) => n,
            }
        )?;
    }

    writeln!(stdout, "Usage Function")?;

    nodes.sort_by(|a, b| {
        let a: u64 = if let Local::Exact(n) = a.local { n } else { 0 };
        let b: u64 = if let Local::Exact(n) = b.local { n } else { 0 };
        b.cmp(&a)
    });

    for node in nodes.iter() {
        let name = rustc_demangle::demangle(&node.name);
        let val: u64 = if let Local::Exact(n) = node.local {
            n
        } else {
            0
        };
        write!(stdout, "{} ", val)?;

        let mut escaper = Escaper::new(&mut stdout);
        writeln!(escaper, "{}", name).ok();
        escaper.error?;
    }
    Ok(())
}

