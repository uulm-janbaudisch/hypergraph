use super::{ParsableHMETIS, SerializeHMETIS};
use crate::{Format, Header};
use nom::character::complete::{space0, space1};
use nom::sequence::tuple;
use nom::IResult;

impl ParsableHMETIS for Header {
    fn parse_hmetis(input: &str) -> IResult<&str, Self> {
        let (input, (_, num_nets, _, num_vertices, _, format, _)) = tuple((
            space0,
            usize::parse_hmetis,
            space1,
            usize::parse_hmetis,
            space0,
            Format::parse_hmetis,
            space0,
        ))(input)?;
        Ok((
            input,
            Header {
                num_nets,
                num_vertices,
                format,
                one_indexed: true,
            },
        ))
    }
}

impl SerializeHMETIS for Header {
    fn serialize_hmetis(&self, output: &mut String) {
        output.push_str(format!("{} {}", self.num_nets, self.num_vertices).as_str());

        if self.format != Format::Unweighted {
            output.push(' ');
            self.format.serialize_hmetis(output);
        }
    }
}
