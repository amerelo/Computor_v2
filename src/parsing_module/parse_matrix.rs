use parsing_module::parse::{ float64, int64 };
use elemt_module::computorelem::{ComputorUnit, ComputorElem};


named!(get_nextfloat<&str, ComputorElem>, do_parse!(
	ws!(tag_s!(",")) >>
	val: alt!(
		ws!(float64) |
		ws!(int64)
 	) >>
	(val)
));

named!(get_vec<&str, Vec<ComputorElem> >, do_parse!(
	res: many1!(
		alt!(
			ws!(float64) |
			ws!(int64) |
			get_nextfloat
		)
	)
	>> (res)
));

named!(get_next_vec<&str, Vec<ComputorElem> >, delimited!(
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

named!(matrix_elem<&str, ComputorElem>, do_parse!(
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
		|mut acc: Vec<Vec<ComputorElem>>, item| {
			acc.push(item);
			acc
		}
	)
	>>
	(ComputorElem{unit: ComputorUnit::MAT(res) })
));

named!(pub matrix<&str, ComputorElem >, ws!(
	delimited!(
		tag_s!("["), matrix_elem, tag_s!("]")
	)
));


#[cfg(test)]
mod tests {
	use super::*;
	use parsing_module::parse::{ ResultKind , test_reslut };

	#[test]
	fn ok_matrix_basic_0() {
		assert_eq!(test_reslut(matrix("[[1,0];[2, 3]]")), ResultKind::OK);
	}

	#[test]
	fn ok_matrix_basic_1() {
		assert_eq!(test_reslut(matrix("[ [ 1 , 0 ] ; [  2  ,  3 ] ]")), ResultKind::OK);
	}

	#[test]
	fn ok_matrix_basic_2() {
		assert_eq!(test_reslut(matrix("[ [ 1 , 0 ] ]")), ResultKind::OK);
	}

	#[test]
	fn error_matrix_basic_0() {
		assert_eq!(test_reslut(matrix("[toto]")), ResultKind::ERROR);
	}

	#[test]
	fn error_matrix_basic_1() {
		assert_eq!(test_reslut(matrix("[[1 , 0 ] , [1 , 2]]")), ResultKind::ERROR);
	}

	#[test]
	fn error_matrix_basic_2() {
		assert_eq!(test_reslut(matrix("[ [ 1 , 0 ] ; [1 , ]]")), ResultKind::ERROR);
	}

	#[test]
	fn error_matrix_0() {
		assert_eq!(test_reslut(matrix("[[1 , 2 ]  [3 , 4]]")), ResultKind::ERROR);
	}
}
