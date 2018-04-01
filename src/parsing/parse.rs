use std::fmt::Debug;
use std::str;
use nom;
use nom::{digit, alphanumeric};
// use std::ops::{ Add, Sub ,Div , Mul};
// , Err,ErrorKind

#[derive(PartialEq, Debug)]
pub enum ResultKind {
	OK,
	ERROR,
	INCOMP,
}

#[derive(PartialEq, Debug)]
pub enum ComputorUnit {
	I64(i64),
	F64(f64),
	VAR(String),
	ATT(String),
	MAT(Vec<ComputorUnit>)
}

#[derive(PartialEq, Debug)]
pub struct ComputorElem {
	pub unit: ComputorUnit,
	// pub mat: Vec<ComputorUnit>,
}

// #[derive(PartialEq, Debug)]
// pub enum ComputorElem {
// 	Unit(ComputorUnit),
// 	Mat(Vec<ComputorUnit>),
// 	// Func(f64),
// }

// impl Add for ComputorElem {
// 	type Output = ComputorElem;
//
// 	fn add(self, other: ComputorElem) -> ComputorElem {
// 		ComputorElem {
// 			unit: self.unit + other.unit
// 		}
// 	}
// }
//
// impl Sub for ComputorElem {
// 	type Output = ComputorElem;
//
// 	fn sub(self, other: ComputorElem) -> ComputorElem {
// 		ComputorElem {
// 			unit: self.unit - other.unit
// 		}
// 	}
// }
//
// impl Mul for ComputorElem {
// 	type Output = ComputorElem;
//
// 	fn mul(self, other: ComputorElem) -> ComputorElem {
// 		ComputorElem {
// 			unit: self.unit * other.unit
// 		}
// 	}
// }
//
// impl Div for ComputorElem {
// 	type Output = ComputorElem;
//
// 	fn div(self, other: ComputorElem) -> ComputorElem {
// 		if other.unit == 0.0 {
// 			panic!("Cannot divide by zero-valued `Rational`!");
// 		}
// 		ComputorElem {
// 			unit: self.unit / other.unit
// 		}
// 	}
// }

named!(signed_digits<&str, &str>,
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
			complete!(pair!(
				tag_s!("."),
				digit
			))
		)
	)
);

named!(pub int64<&str, ComputorElem>, do_parse!(
	elem: map_res!(signed_digits, str::FromStr::from_str) >>
	(ComputorElem{ unit: ComputorUnit::I64(elem) })
));

named!(pub float64<&str, ComputorElem>, do_parse!(
	elem: map_res!(floating_point, str::FromStr::from_str) >>
	(ComputorElem{ unit: ComputorUnit::F64(elem) })
));

named!(pub get_attributor<&str, ComputorElem>, do_parse!(
	elem: alt!(
			tag!("=") |
			tag!("+") |
			tag!("-") |
			tag!("/") |
			tag!("*") |
			tag!("^")
	) >>
	(ComputorElem{ unit: ComputorUnit::ATT( String::from(elem) ) })
));

named!(pub get_var<&str, ComputorElem>, do_parse!(
	elem: fold_many0!(
			alphanumeric,
			String::new(),
			|mut acc: String, v | {
				acc = String::from(v);// str::from_utf8(v).unwrap().to_string();
				acc
			}
		)  >>
		( ComputorElem{ unit: ComputorUnit::VAR( elem ) } )
));

named!(pub select_parser<&str, Vec<ComputorElem> >,
	do_parse!(
		res: many1!(
			alt!(
				ws!(float64) |
				ws!(int64) |
				ws!(get_attributor) |
				ws!(get_var)
			)
		) >>
		(res)
	)
);

// named!(factor<&str, ComputorElem >,
// 	alt!(
// 		ws!(float64) |
// 		ws!(delimited!( tag_s!("("), expr, tag_s!(")") ))
// 	)
// );
//
// named!(term<&str, ComputorElem >, do_parse!(
// 	init: factor >>
// 	res: fold_many0!(
// 		tuple!(
// 			alt!(tag_s!("*") | tag_s!("/")),
// 			factor
// 		),
// 		init,
// 		|acc, v:(_,ComputorElem)| {
// 			if v.0 == "*" {acc * v.1} else {acc / v.1}
// 		}
// 	)
// 	>> (res)
// ));
//
// named!(pub expr<&str, ComputorElem >, do_parse!(
// 	init: term >>
// 	res: fold_many0!(
// 		tuple!(
// 			alt!(tag_s!("+") | tag_s!("-")),
// 			term
// 		),
// 		init,
// 		|acc, v:(_,ComputorElem )| {
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
