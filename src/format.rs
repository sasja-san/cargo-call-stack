
use clap::ValueEnum;

// re-exporting functions
mod top;     pub use top::top;
mod dot;     pub use dot::dot;
mod escaper; pub use escaper::*;



#[derive(ValueEnum, PartialEq, Debug, Clone, Copy)]
pub enum OutputFormat {
    Dot,
    Top,
}


