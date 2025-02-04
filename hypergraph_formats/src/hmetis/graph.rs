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
        Ok((input, Graph { header, nets }))
    }
}

impl SerializeHMETIS for Net {
    fn serialize_hmetis(&self, output: &mut String) {
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

impl SerializeHMETIS for Graph {
    fn serialize_hmetis(&self, output: &mut String) {
        self.header.serialize_hmetis(output);
        output.push('\n');
        self.nets
            .iter()
            .for_each(|net| net.serialize_hmetis(output));
    }
}

#[cfg(test)]
mod test {
    use crate::hmetis::{FromStringHMETIS, ToStringHMETIS};
    use crate::{Format, Graph, Header, Net};

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
            Graph {
                header: Header {
                    num_nets: 4,
                    num_vertices: 7,
                    format: Format::Unweighted,
                },
                nets: vec![
                    Net(vec![1, 2]),
                    Net(vec![1, 7, 5, 6]),
                    Net(vec![5, 6, 4]),
                    Net(vec![2, 3, 4])
                ],
            }
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

        assert_eq!(
            Graph {
                header: Header {
                    num_nets: 4,
                    num_vertices: 7,
                    format: Format::Unweighted,
                },
                nets: vec![
                    Net(vec![1, 2]),
                    Net(vec![1, 7, 5, 6]),
                    Net(vec![5, 6, 4]),
                    Net(vec![2, 3, 4])
                ],
            }
            .to_string_hmetis(),
            expected
        );
    }
}
