use nom::IResult;
use nom::number::complete::be_u16;
use nom::bytes::complete::take;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Needed {
  Unknown,
  Size(u32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Err<E> {
  Incomplete(Needed),
  Error(E),
  Failure(E),
}

pub fn length_value(input: &[u8]) -> IResult<&[u8],&[u8]> {
  let (input, length) = be_u16(input)?;
  take(length)(input)
}

pub fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {

}