use crate::input::lex::core::LexResult;
use crate::input::parse::{Timestamp, Token};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use nom::bytes::complete::take;
use nom::Parser;

fn default_offset() -> FixedOffset {
    FixedOffset::east_opt(0).unwrap()
}

fn lex_datetime(input: &str) -> LexResult<'_, Timestamp> {
    let (input, dt_str) = take(29usize).parse(input)?;

    let dt = match DateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S%.3f%:z") {
        Ok(x) => x,
        Err(_) => {
            return Err(nom::Err::Error(nom::error::make_error(
                dt_str,
                nom::error::ErrorKind::IsNot,
            )))
        }
    };

    Ok((input, dt))
}

fn lex_date(input: &str) -> LexResult<'_, NaiveDate> {
    let (input, datestr) = take(10usize).parse(input)?;

    let date = match NaiveDate::parse_from_str(datestr, "%Y-%m-%d") {
        Ok(x) => x,
        Err(_) => {
            return Err(nom::Err::Error(nom::error::make_error(
                datestr,
                nom::error::ErrorKind::IsNot,
            )))
        }
    };

    Ok((input, date))
}

pub fn lex(input: &str) -> LexResult<'_, Token> {
    let (input, dt) = match lex_datetime(input) {
        Ok(x) => x,
        Err(_) => {
            let (input, date) = lex_date(input)?;
            let ndt = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            let dt = DateTime::from_naive_utc_and_offset(ndt, default_offset());

            (input, dt)
        }
    };

    Ok((input, Token::Timestamp(dt)))
}

#[cfg(test)]
mod test {
    use super::{lex, Token};
    use chrono::DateTime;

    #[rstest::rstest]
    #[case::date("2026-01-01", "2026-01-01 00:00:00.000Z")]
    #[case::timestamp_with_timezone("2026-01-01 00:00:00.000+00:00", "2026-01-01 00:00:00.000Z")]
    fn test_succeeds(#[case] input: &str, #[case] timestamp: &str) {
        let result = lex(input);
        assert_eq!(
            result,
            Ok((
                "",
                Token::Timestamp(
                    DateTime::parse_from_rfc3339(timestamp).expect("Invalid test case.")
                )
            ))
        )
    }
}
