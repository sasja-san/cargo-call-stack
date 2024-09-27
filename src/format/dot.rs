

use core::{
    fmt::Write, 
    str
};
use std::{
    io,
    io::Write as _
};

use crate::prelude::*;
use super::escaper::Escaper;

use petgraph::{Graph, graph::NodeIndex};
// use std::borrow::Cow;



// use super::escaper;


// Font used in the dot graphs
const FONT: &str = "monospace";




pub fn dot(g: &Graph<Node, ()>, cycles: &[Vec<NodeIndex>]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    writeln!(stdout, "digraph {{")?;
    writeln!(stdout, "    node [fontname={} shape=box]", FONT)?;

    for (i, node) in g.raw_nodes().iter().enumerate() {
        let node = &node.weight;

        write!(stdout, "    {} [label=\"", i,)?;

        let mut escaper = Escaper::new(&mut stdout);
        write!(escaper, "{}", rustc_demangle::demangle(&node.name)).ok();
        escaper.error?;

        if let Some(max) = node.max {
            write!(stdout, "\\nmax {}", max)?;
        }

        write!(stdout, "\\nlocal = {}\"", node.local,)?;

        if node.dashed {
            write!(stdout, " style=dashed")?;
        }

        writeln!(stdout, "]")?;
    }

    for edge in g.raw_edges() {
        writeln!(
            stdout,
            "    {} -> {}",
            edge.source().index(),
            edge.target().index()
        )?;
    }

    for (i, cycle) in cycles.iter().enumerate() {
        writeln!(stdout, "\n    subgraph cluster_{} {{", i)?;
        writeln!(stdout, "        style=dashed")?;
        writeln!(stdout, "        fontname={}", FONT)?;
        writeln!(stdout, "        label=\"SCC{}\"", i)?;

        for node in cycle {
            writeln!(stdout, "        {}", node.index())?;
        }

        writeln!(stdout, "    }}")?;
    }

    writeln!(stdout, "}}")
}




