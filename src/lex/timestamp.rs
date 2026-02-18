use super::core::{NomResult, StringScanner};
use crate::tokens::{Timestamp, Token};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use nom::bytes::complete::take;
use nom::Parser;

fn default_offset() -> FixedOffset {
    FixedOffset::east_opt(0).unwrap()
}

fn lex_datetime(input: StringScanner) -> NomResult<Timestamp> {
    let (input, dt_str) = take(29usize).parse(input)?;

    let dt = match DateTime::parse_from_str(dt_str.as_str(), "%Y-%m-%d %H:%M:%S%.3f%:z") {
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

fn lex_date(input: StringScanner) -> NomResult<NaiveDate> {
    let (input, datestr) = take(10usize).parse(input)?;

    let date = match NaiveDate::parse_from_str(datestr.as_str(), "%Y-%m-%d") {
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

pub fn lex(input: StringScanner) -> NomResult<Token> {
    let (input, dt) = match lex_datetime(input.clone()) {
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
        let (_, result) = lex(input.into()).expect("Failed.");
        assert_eq!(
            result,
            Token::Timestamp(DateTime::parse_from_rfc3339(timestamp).expect("Invalid test case."))
        )
    }
}
