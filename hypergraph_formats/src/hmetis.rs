mod comment;
mod format;
mod graph;
mod header;

use crate::Parsable;
use nom::IResult;

/// Enables a structure to be parsed from the hMETIS hypergraph format.
///
/// Only used for internally combining multiple parsers.
/// From the outside, `FromStringHMETIS` is used.
trait ParsableHMETIS {
    /// Parses this structure from the hMETIS hypergraph format.
    fn parse_hmetis(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

// Everything that is parsable by default should also be parsable in the hMETIS context.
impl<T: Parsable> ParsableHMETIS for T {
    fn parse_hmetis(input: &str) -> IResult<&str, Self> {
        Self::parse(input)
    }
}

/// Enables a structure to be parsed from the hMETIS hypergraph format.
pub trait FromStringHMETIS {
    /// Parses this structure from the hMETIS hypergraph format.
    fn from_string_hmetis(input: &str) -> Result<Self, ()>
    where
        Self: Sized;
}

// Everything that is parsable can be constructed from a string.
impl<T: ParsableHMETIS> FromStringHMETIS for T {
    fn from_string_hmetis(input: &str) -> Result<Self, ()> {
        // TODO: propagate error
        let (_remaining, graph) = Self::parse_hmetis(input).unwrap();
        Ok(graph)
    }
}

/// Enables a structure to be serialized into the hMETIS hypergraph format.
///
/// Only used for internally combining multiple serializers.
/// From the outside, `ToStringHMETIS` is used.
trait SerializeHMETIS {
    /// Serializes this structure into the hMETIS hypergraph format.
    fn serialize_hmetis(&self, output: &mut String);
}

/// Enables a structure to be serialized into the hMETIS hypergraph format.
pub trait ToStringHMETIS {
    /// Serializes this structure into the hMETIS hypergraph format.
    fn to_string_hmetis(&self) -> String;
}

// Everything that serializable can be converted into a string.
impl<T: SerializeHMETIS> ToStringHMETIS for T {
    fn to_string_hmetis(&self) -> String {
        let mut buffer = String::new();
        self.serialize_hmetis(&mut buffer);
        buffer
    }
}
