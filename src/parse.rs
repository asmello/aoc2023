use chumsky::prelude::*;

use crate::errors::{GenericError, ParseError};

pub(crate) fn parse<'a, T>(
    input: &'a str,
    parser: impl Parser<'a, &'a str, T, extra::Err<Rich<'a, char>>>,
) -> miette::Result<T> {
    let output = parser.parse(input).into_result().map_err(|errors| {
        ParseError::new(
            input,
            errors
                .into_iter()
                .map(|err| GenericError::new(err.to_string(), err.span().into_range())),
        )
    })?;
    Ok(output)
}
