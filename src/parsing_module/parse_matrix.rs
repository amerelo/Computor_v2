// use parsing::parse::{ float64 };

// named!(get_nextfloat<&str, f64>, do_parse!(
// 	ws!(tag_s!(",")) >>
// 	val: ws!(float64) >>
// 	(val)
// ));
//
// named!(get_vec<&str, Vec<f64> >, do_parse!(
// 	res: many1!(
// 		alt!(
// 			ws!(float64) | get_nextfloat
// 		)
// 	)
// 	>> (res)
// ));
//
// named!(get_next_vec<&str, Vec<f64> >, delimited!(
// 	recognize!(
// 			tuple!(
// 				ws!(tag_s!(";")),
// 				tag_s!("[")
// 			)
// 		),
// 		get_vec,
// 		tag_s!("]")
// 	)
// );
//
// named!(matrix_elem<&str, Vec<Vec<f64> > >, do_parse!(
// 	init: count!(
// 		do_parse!(
// 			tag_s!("[") >>
// 			val: get_vec >>
// 			tag_s!("]") >>
// 			(val)
// 		), 1
// 	) >>
// 	res: fold_many0!(
// 		get_next_vec,
// 		init,
// 		|mut acc: Vec<Vec<f64>>, item| {
// 			acc.push(item);
// 			acc
// 		}
// 	)
// 	>> (res)
// ));
//
// named!(pub matrix<&str, Vec<Vec<f64> > >, ws!(
// 		delimited!(
// 			tag_s!("["), matrix_elem, tag_s!("]")
// 		)
// ));


#[cfg(test)]
mod tests {
	use super::*;
	use parsing::parse::{ ResultKind , test_reslut };

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
