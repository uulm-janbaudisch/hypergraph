use super::SerializePATOH;
use crate::{Format, Header};

impl SerializePATOH for Header {
    fn serialize_patoh(&self, output: &mut String) {
        output.push_str(format!("0 {} {}", self.num_vertices, self.num_nets).as_str());

        if self.format != Format::Unweighted {
            output.push(' ');
            self.format.serialize_patoh(output);
        }
    }
}
