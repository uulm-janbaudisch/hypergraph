use super::comment::Space;
use super::{ParsableHMETIS, SerializeHMETIS};
use crate::{Graph, Header, Net};
use nom::character::complete::space1;
use nom::multi::{many0, separated_list1};
use nom::sequence::tuple;
use nom::IResult;

/// Used for collecting nets contents while not knowing the actual format of the hypergraph,
/// therefore not knowing whether the first number is a vertex or the weight of the net.
type TempNet = Vec<usize>;

impl ParsableHMETIS for TempNet {
    fn parse_hmetis(input: &str) -> IResult<&str, Self> {
        let (input, _) = many0(space1)(input)?;
        separated_list1(space1, usize::parse_hmetis)(input)
    }
}

impl ParsableHMETIS for Graph {
    fn parse_hmetis(input: &str) -> IResult<&str, Self> {
        let (input, (_, header, _, nets, _)) = tuple((
            Space::parse_hmetis,
            Header::parse_hmetis,
            Space::parse_hmetis,
            separated_list1(Space::parse_hmetis, TempNet::parse_hmetis),
            Space::parse_hmetis,
        ))(input)?;

        let nets = nets.into_iter().map(Net).collect();
        Ok((
            input,
            Graph {
                header,
                nets,
                vertex_weights: Vec::new(),
                net_weights: Vec::new(),
            },
        ))
    }
}

impl SerializeHMETIS for Net {
    fn serialize_hmetis(&self, output: &mut String) {
        output.push_str(&self.to_string());
    }
}

impl SerializeHMETIS for Graph {
    fn serialize_hmetis(&self, output: &mut String) {
        let serialize_vertex_weights = self.header.format.contains_vertex_weights();
        let serialize_net_weights = self.header.format.contains_net_weights();

        self.header.serialize_hmetis(output);

        output.push('\n');
        self.nets.iter().enumerate().for_each(|(index, net)| {
            // Optionally serialize the net weights.
            if serialize_net_weights {
                output.push_str(&self.net_weights[index].to_string());
                output.push(' ');
            }

            net.serialize_hmetis(output);
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
    use crate::hmetis::{FromStringHMETIS, ToStringHMETIS};
    use crate::{Format, Graph, Header, Net};

    fn graph_unweighted() -> Graph {
        Graph {
            header: Header {
                num_nets: 4,
                num_vertices: 7,
                format: Format::Unweighted,
                one_indexed: true,
            },
            nets: vec![
                Net(vec![1, 2]),
                Net(vec![1, 7, 5, 6]),
                Net(vec![5, 6, 4]),
                Net(vec![2, 3, 4]),
            ],
            vertex_weights: vec![],
            net_weights: vec![],
        }
    }

    fn graph_vertex_weights() -> Graph {
        let mut graph = graph_unweighted();
        graph.header.format = Format::VertexWeights;
        graph.vertex_weights = vec![1, 5, 2, 2, 2, 1, 3];
        graph
    }

    #[test]
    fn parse_unweighted() {
        let input = r#"
% a comment
4 7
1 2
% another comment

1 7 5 6
5 6 4
2 3 4
        "#;

        assert_eq!(
            Graph::from_string_hmetis(input).unwrap(),
            graph_unweighted()
        );
    }

    #[test]
    fn serialize_unweighted() {
        let expected = r#"4 7
1 2
1 7 5 6
5 6 4
2 3 4
"#;

        assert_eq!(graph_unweighted().to_string_hmetis(), expected);
    }

    #[test]
    fn serialize_vertex_weighted() {
        let expected = r#"4 7 10
1 2
1 7 5 6
5 6 4
2 3 4
1
5
2
2
2
1
3
"#;

        assert_eq!(graph_vertex_weights().to_string_hmetis(), expected);
    }
}
