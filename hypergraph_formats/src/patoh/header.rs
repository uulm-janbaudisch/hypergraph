use super::SerializePATOH;
use crate::Header;

impl SerializePATOH for Header {
    fn serialize_patoh(&self, output: &mut String) {
        if self.one_indexed {
            output.push('1');
        } else {
            output.push('0');
        }

        output.push_str(format!(" {} {}", self.num_vertices, self.num_nets).as_str());
    }
}
