use super::ParsableHMETIS;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, newline, space0};
use nom::combinator::recognize;
use nom::multi::{many0, many_till};
use nom::sequence::preceded;
use nom::IResult;

fn empty(input: &str) -> IResult<&str, &str> {
    recognize(many_till(space0, newline))(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    recognize(preceded(tag("%"), many_till(anychar, newline)))(input)
}

/// Can be (multiple of): newline, empty line or comment.
pub struct Space;

impl ParsableHMETIS for Space {
    fn parse_hmetis(input: &str) -> IResult<&str, Self> {
        let (input, _) = many0(alt((recognize(newline), empty, comment)))(input)?;
        Ok((input, Space))
    }
}
