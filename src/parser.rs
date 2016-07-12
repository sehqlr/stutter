extern crate nom;
use nom::{IResult, digit, eof};
use nom::IResult::*;

pub fn define_parsers() {
    named!(number,
           many1!(digit)
    );

    named!(operator,
           is_a!("+-*/")
    );

    named!(expr,
        alt!(
            value!(number)
        |   delimited!(
                char!('('),
                preceded!(
                    operator,
                    many1!(expr)
                ),
                char!(')')
            )
        )
    );

    named!(stutter,
        terminated!(
            preceded!(
                value!(operator),
                many0!(expr)
            ),
        eof)
    );

}
