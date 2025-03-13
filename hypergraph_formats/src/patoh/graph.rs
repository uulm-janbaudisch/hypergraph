use super::SerializePATOH;
use crate::{Format, Graph, Net};

impl SerializePATOH for Net {
    fn serialize_patoh(&self, output: &mut String) {
        output.push_str(&self.to_string());
    }
}

impl SerializePATOH for Graph {
    fn serialize_patoh(&self, output: &mut String) {
        let serialize_vertex_weights = self.header.format.contains_vertex_weights();
        let serialize_net_weights = self.header.format.contains_net_weights();

        self.header.serialize_patoh(output);

        // The pin count can only be calculated by the graph itself.
        output.push(' ');
        output.push_str(&self.pin_count().to_string());

        if self.header.format != Format::Unweighted {
            output.push(' ');
            self.header.format.serialize_patoh(output);
        }

        output.push('\n');

        self.nets.iter().enumerate().for_each(|(index, net)| {
            // Optionally serialize the net weights.
            if serialize_net_weights {
                output.push_str(&self.net_weights[index].to_string());
                output.push(' ');
            }

            net.serialize_patoh(output);
            output.push('\n');
        });

        // Optionally serialize the vertex weights.
        if serialize_vertex_weights {
            self.vertex_weights.iter().for_each(|weight| {
                output.push_str(&weight.to_string());
                output.push('\n');
            });
        }
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
                    one_indexed: false,
                },
                nets: vec![
                    Net(vec![2, 3, 5, 6, 9]),
                    Net(vec![0, 1]),
                    Net(vec![0, 1, 2, 3]),
                ],
                vertex_weights: vec![],
                net_weights: vec![],
            }
            .to_string_patoh(),
            expected
        );
    }
}
