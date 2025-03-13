//! This crate implements formats for parsing and serializing a hypergraph.
//!
//! Each component implements `ToString<format>` and `FromString<formt>` to serialize and parse it.

#[cfg(feature = "cnf")]
pub mod cnf;
mod format;
mod graph;
mod header;

/// The hypergraph format as specified by [hMETIS](https://course.ece.cmu.edu/~ee760/760docs/hMetisManual.pdf).
pub mod hmetis;

/// The hypergraph format as specified by [PaToH](https://faculty.cc.gatech.edu/~umit/PaToH/manual.pdf).
pub mod patoh;

pub use format::Format;
pub use graph::{Graph, Net, NetIndex, VertexIndex, Weight};
pub use header::Header;
use nom::bytes::complete::take_while;
use nom::combinator::map_res;
use nom::{AsChar, IResult};
use std::str::FromStr;

trait Parsable {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parsable for usize {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(take_while(char::is_dec_digit), usize::from_str)(input)
    }
}
