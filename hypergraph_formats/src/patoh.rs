mod format;
mod graph;
mod header;

/// Enables a structure to be serialized into the PaToH hypergraph format.
///
/// Only used for internally combining multiple serializers.
/// From the outside, `ToStringPATOH` is used.
trait SerializePATOH {
    /// Serializes this structure into the PaToH hypergraph format.
    fn serialize_patoh(&self, output: &mut String);
}

/// Enables a structure to be serialized into the PaToH hypergraph format.
pub trait ToStringPATOH {
    /// Serializes this structure into the PaToH hypergraph format.
    fn to_string_patoh(&self) -> String;
}

// Everything that serializable can be converted into a string.
impl<T: SerializePATOH> ToStringPATOH for T {
    fn to_string_patoh(&self) -> String {
        let mut buffer = String::new();
        self.serialize_patoh(&mut buffer);
        buffer
    }
}
