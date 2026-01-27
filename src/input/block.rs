use super::common::{InputParser, ParserResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::rest;
use nom::error::ErrorKind;
use nom::sequence::terminated;
use nom::Parser;

pub struct Block(String);

pub struct BlockParser {}

fn is_line_empty(line: &str) -> bool {
    line.replace(" ", "").is_empty()
}

pub fn parse_line<'a>(input: &'a str) -> ParserResult<'a, &'a str> {
    alt((terminated(take_until("\n"), tag("\n")), rest)).parse(input)
}

pub fn parse_empty_line(input: &str) -> ParserResult<'_, ()> {
    let (input, line) = parse_line(input)?;
    if !is_line_empty(line) {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            ErrorKind::IsNot,
        )));
    }

    Ok((input, ()))
}

impl InputParser<Block> for BlockParser {
    fn parse(input: &str) -> ParserResult<'_, Block> {
        let mut i = input;
        let mut block = "".to_string();

        loop {
            if i.is_empty() {
                return Ok((i, Block(block)));
            }

            match parse_empty_line(i) {
                Ok((next_i, _)) => {
                    i = next_i;
                }
                Err(_) => {
                    break;
                }
            }
        }

        let (next_i, initial_line) = parse_line(i)?;
        i = next_i;
        block = initial_line.to_string();

        loop {
            if i.is_empty() {
                return Ok((i, Block(block)));
            }

            if let Ok((next_i, _)) = parse_empty_line(i) {
                i = next_i;
                continue;
            }

            let (next_i, next_line) = parse_line(i)?;
            if !next_line.starts_with("  ") {
                break;
            }

            i = next_i;
            block += next_line;
        }

        Ok((i, Block(block)))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use indoc::indoc;

    #[test]
    fn test_empty() {
        let input = indoc! {"

        "};
        BlockParser::parse(input).expect("Failed.");
    }

    #[test]
    fn test_ok_simple() {
        let input = indoc! {"
            foo
              a
              b
        "};
        let (rest, Block(block)) = BlockParser::parse(input).expect("Failed.");
        assert!(block.starts_with("foo"));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_ok_if_block_ends_on_eof() {
        let input = indoc! {"
            foo
              a
              b"};
        let (rest, Block(block)) = BlockParser::parse(input).expect("Failed.");
        assert!(block.starts_with("foo"));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_ok_consumes_preceding_whitespace() {
        let input = indoc! {"
            
              
                 
            foo
              a
              b
        "};
        let (rest, Block(block)) = BlockParser::parse(input).expect("Failed.");
        assert!(block.starts_with("foo"));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_ok_consumes_following_whitespace() {
        let input = indoc! {"
            foo
              a
              b
               
              

              
        "};
        let (rest, Block(block)) = BlockParser::parse(input).expect("Failed.");
        assert!(block.starts_with("foo"));
        assert_eq!(rest, "");
    }
    #[test]
    fn test_ok_stops_before_next_block() {
        let input = indoc! {"
            foo
              a
              b
            bar
              baz
        "};
        let (rest, Block(block)) = BlockParser::parse(input).expect("Failed.");
        assert!(block.starts_with("foo"));
        assert!(rest.starts_with("bar"));
    }
}
