use super::SerializePATOH;
use crate::Format;

impl SerializePATOH for Format {
    fn serialize_patoh(&self, output: &mut String) {
        output.push_str(match self {
            Format::Unweighted => "",
            Format::NetWeights => "2",
            Format::VertexWeights => "1",
            Format::Weighted => "3",
        });
    }
}
