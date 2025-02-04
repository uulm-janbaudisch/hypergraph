use super::SerializePATOH;
use crate::{Graph, Net};

impl SerializePATOH for Net {
    fn serialize_patoh(&self, output: &mut String) {
        let last = self.0.len() - 1;
        self.0.iter().enumerate().for_each(|(i, vertex)| {
            output.push_str(vertex.to_string().as_str());

            // All but the last vertex should have a space appended as a separator.
            if i != last {
                output.push(' ');
            }
        });

        output.push('\n');
    }
}

impl SerializePATOH for Graph {
    fn serialize_patoh(&self, output: &mut String) {
        self.header.serialize_patoh(output);

        // The pin count can only be calculated by the graph itself.
        output.push(' ');
        output.push_str(&self.pin_count().to_string());
        output.push('\n');

        self.nets.iter().for_each(|net| net.serialize_patoh(output));
    }
}

#[cfg(test)]
mod test {
    use crate::patoh::ToStringPATOH;
    use crate::{Format, Graph, Header, Net};

    #[test]
    fn serialize_unweighted() {
        let expected = r#"0 10 3 11
2 3 5 6 9
0 1
0 1 2 3
"#;

        assert_eq!(
            Graph {
                header: Header {
                    num_nets: 3,
                    num_vertices: 10,
                    format: Format::Unweighted,
                },
                nets: vec![
                    Net(vec![2, 3, 5, 6, 9]),
                    Net(vec![0, 1]),
                    Net(vec![0, 1, 2, 3]),
                ],
            }
            .to_string_patoh(),
            expected
        );
    }
}
