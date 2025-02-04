use super::{ParsableHMETIS, SerializeHMETIS};
use crate::Format;
use nom::bytes::complete::take_while;
use nom::combinator::map_res;
use nom::{AsChar, IResult};

impl TryFrom<&str> for Format {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "" => Ok(Format::Unweighted),
            "1" => Ok(Format::NetWeights),
            "10" => Ok(Format::VertexWeights),
            "11" => Ok(Format::Weighted),
            _ => Err(()),
        }
    }
}

impl ParsableHMETIS for Format {
    fn parse_hmetis(input: &str) -> IResult<&str, Self> {
        map_res(take_while(char::is_dec_digit), Format::try_from)(input)
    }
}

impl SerializeHMETIS for Format {
    fn serialize_hmetis(&self, output: &mut String) {
        output.push_str(match self {
            Format::Unweighted => "",
            Format::NetWeights => "1",
            Format::VertexWeights => "10",
            Format::Weighted => "11",
        });
    }
}
