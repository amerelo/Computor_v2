use std::str::FromStr;
use std::fmt::Debug;
use nom;
use nom::{digit};
// , Err,ErrorKind

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

// #################################### matirx
named!(get_nextfloat<&str, f64>, do_parse!(
	ws!(tag_s!(",")) >>
	val: ws!(float64) >>
	(val)
));

named!(get_vec<&str, Vec<f64> >, do_parse!(
	res: many1!(
		alt!(
			ws!(float64) | get_nextfloat
		)
	)
	>> (res)
));

named!(get_next_vec<&str, Vec<f64> >, delimited!(
	recognize!(
			tuple!(
				ws!(tag_s!(";")),
				tag_s!("[")
			)
		),
		get_vec,
		tag_s!("]")
	)
);

named!(matrix_elem<&str, Vec<Vec<f64> > >, do_parse!(
	init: count!(
		do_parse!(
			tag_s!("[") >>
			val: get_vec >>
			tag_s!("]") >>
			(val)
		), 1
	) >>
	res: fold_many0!(
		get_next_vec,
		init,
		|mut acc: Vec<Vec<f64>>, item| {
			acc.push(item);
			acc
		}
	)
	>> (res)
));

named!(matrix<&str, Vec<Vec<f64> > >, ws!(
		delimited!(
			tag_s!("["), matrix_elem, tag_s!("]")
		)
));

// ####################################

// TEST FUNCTION
// rest.drain(..());
fn dump<T: Debug>(res: nom::IResult<&str,T>)
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

// TEST FUNCTION
// TODO Change for enum
fn test_reslut<T: Debug>(res: nom::IResult<&str,T>) -> i32
{
	match res {
		nom::IResult::Done(rest, _) => {
			if !rest.is_empty() {
				3
			}
			else {
				1
			}
		},
		nom::IResult::Incomplete(_) => {
			2
		},
		nom::IResult::Error(_err) => {
			3
		},
	}
}

// TEST FUNCTION
pub fn test_nom(name : &mut String)
{
	// dump(expr(name));
	dump(matrix(name));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ok_matrix_basic_0() {
		assert_eq!(test_reslut(matrix("[[1,0];[2, 3]]")), 1);
	}

	#[test]
	fn ok_matrix_basic_1() {
		assert_eq!(test_reslut(matrix("[ [ 1 , 0 ] ; [  2  ,  3 ] ]")), 1);
	}

	#[test]
	fn ok_matrix_basic_2() {
		assert_eq!(test_reslut(matrix("[ [ 1 , 0 ] ]")), 1);
	}

	#[test]
	fn error_matrix_basic_0() {
		assert_eq!(test_reslut(matrix("[toto]")), 3);
	}

	#[test]
	fn error_matrix_basic_1() {
		assert_eq!(test_reslut(matrix("[[1 , 0 ] , [1 , 2]]")), 3);
	}

	#[test]
	fn error_matrix_basic_2() {
		assert_eq!(test_reslut(matrix("[ [ 1 , 0 ] ; [1 , ]]")), 3);
	}

	#[test]
	fn error_matrix_0() {
		assert_eq!(test_reslut(matrix("[[1 , 2 ]  [3 , 4]]")), 3);
	}
}
