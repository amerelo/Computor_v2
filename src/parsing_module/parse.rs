use std::fmt::Debug;
use std::str;
use nom;
use nom::{digit, alpha};
// , Err,ErrorKind

use elemt_module::computorelem::{ComputorUnit, ComputorElem};
use parsing_module::{ parse_matrix::matrix , parse_func::get_func };

#[derive(PartialEq, Debug)]
pub enum ResultKind {
	OK,
	ERROR,
	INCOMP,
}

named!(signed_digits<&str, &str>, recognize!(
	tuple!(
		opt!(alt!(tag_s!("+") | tag_s!("-"))),
		digit
	)
));

named!(floating_point<&str,&str>, recognize!(
	tuple!(
		signed_digits,
		complete!(pair!(
			tag_s!("."),
			digit
		))
	)
));

named!(pub int64<&str, ComputorElem>, do_parse!(
	elem: map_res!(signed_digits, str::FromStr::from_str) >>
	(ComputorElem{unit: ComputorUnit::I64(elem, false) })
));

named!(pub float64<&str, ComputorElem>, do_parse!(
	elem: map_res!(floating_point, str::FromStr::from_str) >>
	(ComputorElem{unit: ComputorUnit::F64(elem, false) })
));

named!(pub get_imaginari<&str, ComputorElem>, do_parse!(
	elem: tag!("i") >>
	(ComputorElem{ unit: ComputorUnit::I64(1, true) })
));

named!(pub get_new<&str, ComputorElem>, do_parse!(
	elem: tuple!(
		ws!(get_var),
		ws!(tag!("="))
	) >>
	(ComputorElem{ unit: ComputorUnit::NEWVAR( elem.0.var_to_string())})
));

// TODO: fix show
named!(pub get_show<&str, ComputorElem>, do_parse!(
	ws!(tag!("=")) >>
	ws!(tag!("?")) >>
	(ComputorElem{ unit: ComputorUnit::SHOW })
));

named!(pub get_attributor<&str, ComputorElem>, do_parse!(
	elem: alt!(
			tag!("+") |
			tag!("-") |
			tag!("/") |
			tag!("**") |
			tag!("*") |
			tag!("%") |
			tag!("^")
	) >>
	(ComputorElem{ unit: ComputorUnit::ATT( String::from(elem) ) })
));

named!(pub get_parentheses<&str, ComputorElem>, do_parse!(
	elem: alt!(
			tag!("(") |
			tag!(")")
	) >>
	(ComputorElem{ unit: ComputorUnit::ATT( String::from(elem) ) })
));

named!(pub get_var<&str, ComputorElem>, do_parse!(
	init: fold_many1!(
			alpha,
			String::new(),
			|mut _acc: String, v | {
				_acc = String::from(v);// str::from_utf8(v).unwrap().to_string();
				_acc
			}
	) >>
	elem: fold_many0!(
			digit,
			init,
			|mut _acc: String, v | {
				// _acc = String::from(v);// str::from_utf8(v).unwrap().to_string();
				_acc.push_str(v);
				_acc
			}
	) >>
	(ComputorElem{ unit: ComputorUnit::VAR( elem ) } )
));

named!(priority_par<&str, ComputorElem>, do_parse!(
 	elem: delimited!( tag_s!("("), 
		do_parse!(
		res: many1!(
			alt_complete!(
				ws!(get_imaginari) |
				ws!(priority_par) |
				ws!(float64) |
				ws!(int64) |
				ws!(get_attributor)
			)
		) >>
		(res)	
	), tag_s!(")") ) >>
	(ComputorElem{ unit: ComputorUnit::VECT( elem ) } )
));

named!(pub vectorised<&str, Vec<ComputorElem> >, do_parse!(
	res: many1!(
		alt_complete!(
			ws!(get_imaginari) |
			ws!(priority_par) |
			ws!(float64) |
			ws!(int64) |
			ws!(get_attributor) |
			ws!(get_parentheses)
		)
	) >>
	(res)	
));

named!(pub parser_elems<&str, Vec<ComputorElem> >, do_parse!(
	res: many1!(
		alt_complete!(
			ws!(get_imaginari) |
			ws!(get_func) |
			ws!(matrix) |
			ws!(float64) |
			ws!(int64) |
			ws!(get_new) |
			ws!(get_attributor) |
			ws!(get_parentheses) |
			ws!(get_var) |
			ws!(get_show)
		)
	) >>
	(res)
));

named!(pub atribut_var<&str, Vec<ComputorElem> >, do_parse!(
	fold: fold_many0!(
		expr,
		Vec::<ComputorElem>::new(), // fold1
		|mut acc:Vec<ComputorElem>, item| {
			acc.push(item);
			acc
		}
	) >>
	(fold)
));

named!(factor<&str, ComputorElem>, alt!(
	ws!(float64) |
	ws!(int64) |
	ws!(delimited!( tag_s!("("), expr, tag_s!(")") ))
));

named!(term<&str, ComputorElem >, do_parse!(
	init: factor >>
	res: fold_many0!(
		tuple!(
			alt!(tag_s!("*") | tag_s!("/")),
			factor
		),
		init,
		|acc, v:(_,ComputorElem)| {
			if v.0 == "*" {acc * v.1} else {acc / v.1}
		}
	)
	>> (res)
));

named!(pub expr<&str, ComputorElem >, do_parse!(
	init: term >>
	res: fold_many0!(
		tuple!(
			alt!(tag_s!("+") | tag_s!("-")),
			term
		),
		init,
		|acc, v:(_,ComputorElem )| {
			if v.0 == "+" {acc + v.1} else {acc - v.1}
		}
	)
	>> (res)
));

pub fn test_reslut<T: Debug>(res: nom::IResult<&str,T>) -> ResultKind
{
	match res {
		nom::IResult::Done(rest, _) => {
			if !rest.is_empty() {
				ResultKind::ERROR
			} else {
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
