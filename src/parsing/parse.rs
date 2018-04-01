use std::str::FromStr;
use std::fmt::Debug;
use nom;
use nom::{digit};
// , Err,ErrorKind

#[derive(PartialEq, Debug)]
pub enum ResultKind {
	OK,
	ERROR,
	INCOMP,
}

named!(signed_digits<&str, &str	>,
	recognize!(
		tuple!(
			opt!(alt!(tag_s!("+") | tag_s!("-"))),
			digit
		)
	)
);

named!(floating_point<&str,&str>,
	recognize!(
		tuple!(
			signed_digits,
			opt!(complete!(pair!(
				tag_s!("."),
				digit
			)))
		)
	)
);

named!(pub float64<&str, f64>,
	map_res!(floating_point, FromStr::from_str)
);

// named!(factor<&str, f64>,
// 	alt!(
// 		ws!(float64) |
// 		ws!(delimited!( tag_s!("("), expr, tag_s!(")") ))
// 	)
// );
//
// named!(term<&str, f64>, do_parse!(
// 	init: factor >>
// 	res: fold_many0!(
// 		tuple!(
// 			alt!(tag_s!("*") | tag_s!("/")),
// 			factor
// 		),
// 		init,
// 		|acc, v:(_,f64)| {
// 			if v.0 == "*" {acc * v.1} else {acc / v.1}
// 		}
// 	)
// 	>> (res)
// ));
//
// named!(expr<&str, f64>, do_parse!(
// 	init: term >>
// 	res: fold_many0!(
// 		tuple!(
// 			alt!(tag_s!("+") | tag_s!("-")),
// 			term
// 		),
// 		init,
// 		|acc, v:(_,f64)| {
// 			if v.0 == "+" {acc + v.1} else {acc - v.1}
// 		}
// 	)
// 	>> (res)
// ));

// TEST FUNCTION
// rest.drain(..());
pub fn dump<T: Debug>(res: nom::IResult<&str,T>)
{
	match res {
			nom::IResult::Done(rest, value) => {
				if !rest.is_empty() {
					println!("invalid command> {:?}", rest)
				} else {
					println!("Done {:?}", value)
				}
			},
			nom::IResult::Incomplete(needed) => { println!("Needed {:?}", needed) },
			nom::IResult::Error(err) => { println!("Err {:?}", err ) },
	}
}

#[allow(dead_code)]
pub fn test_reslut<T: Debug>(res: nom::IResult<&str,T>) -> ResultKind
{
	match res {
		nom::IResult::Done(rest, _) => {
			if !rest.is_empty() {
				ResultKind::ERROR
			}
			else {
				ResultKind::OK
			}
		},
		nom::IResult::Incomplete(_) => {
			ResultKind::INCOMP
		},
		nom::IResult::Error(_err) => {
			ResultKind::ERROR
		},
	}
}
