#[macro_use]
extern crate nom;

use std::str::FromStr;
use std::fmt::Debug;
// use std::num::ParseIntError;

use nom::{Err,ErrorKind};
use nom::digit;

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

named!(float64<&str, f64>,
	map_res!(floating_point, FromStr::from_str)
);

named!(factor<&str, f64>,
	alt!(
		ws!(float64) |
		ws!(delimited!( tag_s!("("), expr, tag_s!(")") ))
	)
);

named!(term<&str, f64>, do_parse!(
	init: factor >>
	res: fold_many0!(
		tuple!(
			alt!(tag_s!("*") | tag_s!("/")),
			factor
		),
		init,
		|acc, v:(_,f64)| {
			if v.0 == "*" {acc * v.1} else {acc / v.1}
		}
	)
	>> (res)
));

named!(expr<&str, f64>, do_parse!(
	init: term >>
	res: fold_many0!(
		tuple!(
			alt!(tag_s!("+") | tag_s!("-")),
			term
		),
		init,
		|acc, v:(_,f64)| {
			if v.0 == "+" {acc + v.1} else {acc - v.1}
		}
	)
	>> (res)
));

named!(get_nextfloat<&str, f64>, do_parse!(
	ws!(tag_s!(",")) >>
	val: ws!(float64) >>
	(val)
));

named!(get_float<&str, f64> , alt!(
	float64 |
	get_nextfloat
));


// many1!( ws!(tag!("hola")) )
named!(get_vec<&str, Vec<f64> >, do_parse!(
	res: many1!(
		get_float
	)
	>> (res)
));

named!(matrix<&str, Vec<f64> >, do_parse!(
	ws!(tag!("[")) >>
	res: get_vec >>
	ws!(tag!("]")) >>
	(res)
));

// rest.drain(..());
fn dump<T: Debug>(res: nom::IResult<&str,T>)
{
	match res {
			nom::IResult::Done(rest, value) => {
				if !rest.is_empty() {
					println!("invalid command> {}", rest)
				} else {
					println!("Done {:?}", value)
				}
			},
			nom::IResult::Incomplete(needed) => { println!("Needed {:?}", needed) },
			nom::IResult::Error(err) => { println!("Err {:?}", err ) },
	}
}

fn test_nom(name : &mut String)
{
	// dump(expr(name));
	dump(matrix(name));
}

fn main()
{
	println!("Welcome to computor_v2");
	let mut name = String::new();

	loop
	{
		std::io::stdin().read_line(&mut name).ok().expect("Failed to read line");
		test_nom(&mut name);
		name.clear();
	}
}
