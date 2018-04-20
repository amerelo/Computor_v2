use std::ops::{ Add, Sub, Div, Mul, BitXor, Rem};
use std::fmt::{ Display, Formatter, Result};

#[derive(PartialEq, Debug, Clone)] 
pub enum ComputorUnit {
	I64(i64, bool),
	F64(f64, bool),
	VAR(String), // <- set pow
	ATT(String),
	MAT(Vec<Vec<ComputorElem>>),
	VECT(Vec<ComputorElem>),
	FUNC(String, String),
	NEWVAR(String),
	SHOW,
	NONE
}

pub enum ComputorAction {
	ADD,
	SUB,
	MUL,
	DIV
}

#[derive(PartialEq, Debug)] 
pub struct ComputorOperand {
	pub is_float: (bool, bool),
	pub is_img: (bool, bool),
	pub is_vect: (bool, bool),
	pub valid: bool
}

#[derive(PartialEq, Debug, Clone)]
pub struct ComputorElem {
	pub unit: ComputorUnit,
}

impl ComputorElem {

	pub fn vec_to_vec_operation(elem1: ComputorElem, elem2: ComputorElem, info: ComputorOperand, action: ComputorAction) -> ComputorElem
	{
		let mut vec1: Vec<ComputorElem> = Vec::new();
		let mut vec2: Vec<ComputorElem> = Vec::new();
		let mut varf = (0.0, 0.0);
		let mut vari = (0, 0);
		let mut pinfo;

		let mut stack: Vec<ComputorElem> = Vec::new();

		if info.is_vect.0 == false { vec1 = vec![elem1]; } else { if let ComputorUnit::VECT(elems) = elem1.unit {vec1 = elems }}
		if info.is_vect.1 == false { vec2 = vec![elem2]; } else { if let ComputorUnit::VECT(elems) = elem2.unit {vec2 = elems }}

		// println!("vec 1 ----- {:#?}", vec1);
		// println!("vec 2 ----- {:#?}", vec2);

		match action {
			ComputorAction::ADD => {
				while let Some(top) = vec2.pop() {
					// if let Some(operation) = vec2.pop() {}
					for elem in vec1.iter_mut() {
						pinfo = elem.get_type_vars(&top, &mut varf, &mut vari);
						if pinfo.valid && pinfo.is_img.0 == pinfo.is_img.1 && ((pinfo.is_float.0 || pinfo.is_float.1) || (!pinfo.is_float.0 && !pinfo.is_float.1)) {
							*elem = (*elem).clone() + top;
							break;
						}
					}
				}
			},
			ComputorAction::SUB => {
				while let Some(top) = vec2.pop() {
					// if let Some(operation) = vec2.pop() {}
					for elem in vec1.iter_mut() {
						pinfo = elem.get_type_vars(&top, &mut varf, &mut vari);
						if pinfo.valid && pinfo.is_img.0 == pinfo.is_img.1 && ((pinfo.is_float.0 || pinfo.is_float.1) || (!pinfo.is_float.0 && !pinfo.is_float.1)) {
							*elem = (*elem).clone() - top;
							break;
						}
					}
				}
			},
			ComputorAction::MUL => {
				while let Some(top) = vec2.pop() {
					// if let Some(operation) = vec2.pop() {}
					for elem in vec1.iter_mut() {
						pinfo = elem.get_type_vars(&top, &mut varf, &mut vari);
						if pinfo.valid && ((pinfo.is_float.0 || pinfo.is_float.1) || (!pinfo.is_float.0 && !pinfo.is_float.1)) {
							*elem = (*elem).clone() * top.clone();
						}
					}
				}
			},
			ComputorAction::DIV => {
				while let Some(top) = vec2.pop() {
					// if let Some(operation) = vec2.pop() {}
					for elem in vec1.iter_mut() {
						pinfo = elem.get_type_vars(&top, &mut varf, &mut vari);
						if pinfo.valid && ((pinfo.is_float.0 || pinfo.is_float.1) || (!pinfo.is_float.0 && !pinfo.is_float.1)) {
							*elem = (*elem).clone() / top.clone();
						}
					}
				}
			}
		}
		return ComputorElem { unit: ComputorUnit::VECT(vec1) };
	}
	
	pub fn get_type_vars(&self, other: &ComputorElem, varf: &mut (f64, f64), vari: &mut (i64, i64)) -> ComputorOperand
	{
		let mut eleminfo = ComputorOperand {
			is_float: (false, false),
			is_img: (false, false),
			is_vect: (false, false),
			valid: true
		};

		if let ComputorUnit::F64(val, img) = self.unit { varf.0 = val; eleminfo.is_float.0 = true; eleminfo.is_img.0 = img}
		else if let ComputorUnit::I64(val, img) = self.unit { vari.0 = val; eleminfo.is_float.0 = false; eleminfo.is_img.0 = img}
		else if let ComputorUnit::VECT(ref _elems) = self.unit { eleminfo.is_vect.0 = true }
		else { eleminfo.valid = false ; return eleminfo }

		if let ComputorUnit::F64(val, img) = other.unit { varf.1 = val; eleminfo.is_float.1 = true; eleminfo.is_img.1 = img}
		else if let ComputorUnit::I64(val, img) = other.unit { vari.1 = val; eleminfo.is_float.1 = false; eleminfo.is_img.1 = img}
		else if let ComputorUnit::VECT(ref _elems) = self.unit { eleminfo.is_vect.1 = true }
		else { eleminfo.valid = false ; return eleminfo }


		if eleminfo.is_float.0 == true || eleminfo.is_float.1 == true {
			if eleminfo.is_float.0 == true && eleminfo.is_float.1 == false { varf.1 = vari.1 as f64; }
			else if eleminfo.is_float.0 == false && eleminfo.is_float.1 == true { varf.0 = vari.0 as f64; }
			eleminfo.is_float = (true, true);
		}
		return eleminfo;
	}
}

impl Display for ComputorElem {
	fn fmt(&self, f: &mut Formatter) -> Result {

		match self.unit {
			ComputorUnit::I64(var, img) => {
				let mut res = var.to_string();
				
				if img { res.push('i'); }
				res.push(' ');
				return write!(f, "{}", res.to_string());
			},
			ComputorUnit::F64(var, img) => {
				let mut res = var.to_string();
				
				if img { res.push('i'); }
				res.push(' ');
				return write!(f, "{}", res.to_string());
			},
			ComputorUnit::VAR(ref var) => return write!(f, "{}", var),
			ComputorUnit::ATT(ref var) => return write!(f, "{}", var),
			ComputorUnit::NEWVAR(ref var) => return write!(f, "{}", var),
			ComputorUnit::SHOW => return write!(f, "=?"),
			ComputorUnit::FUNC(ref name, ref var) => return write!(f, "{}({})", name, var),
			ComputorUnit::MAT(ref _var) => return write!(f, "NONE"), //NONE
			ComputorUnit::VECT(ref _var) => return write!(f, "NONE"), //NONE
			ComputorUnit::NONE => return write!(f, "NONE"), //NONE
		}
	}
}

impl Add for ComputorElem {
	type Output = ComputorElem;

	fn add(self, other: ComputorElem) -> ComputorElem {
		let mut varf = (0.0, 0.0);
		let mut vari = (0, 0);

		let elems_info = self.get_type_vars(&other, &mut varf, &mut vari);
		match elems_info {
			ComputorOperand {valid, .. } if !valid => ComputorElem { unit: ComputorUnit::NONE },
			ComputorOperand {is_vect, .. } if is_vect.0 || is_vect.1 => 
				ComputorElem::vec_to_vec_operation(self, other, elems_info, ComputorAction::ADD),
			ComputorOperand {is_img, .. } if is_img.0 != is_img.1 =>
				ComputorElem { unit: ComputorUnit::VECT( vec![self, ComputorElem{ unit: ComputorUnit::ATT("+".to_string()) } , other] ) },
			ComputorOperand {is_float, is_img, .. } if is_float.0 || is_float.1 =>
				ComputorElem { unit: ComputorUnit::F64(varf.0 + varf.1, is_img.0) },
			ComputorOperand {is_float, is_img, .. } if !is_float.0 && !is_float.1 =>
				ComputorElem { unit: ComputorUnit::I64(vari.0 + vari.1, is_img.0) },
			_ => ComputorElem { unit: ComputorUnit::NONE }
		}
	}
}

impl Sub for ComputorElem {
	type Output = ComputorElem;

	fn sub(self, other: ComputorElem) -> ComputorElem {
		let mut varf = (0.0, 0.0);
		let mut vari = (0, 0);
		
		let elems_info = self.get_type_vars(&other, &mut varf, &mut vari);
		match elems_info {
			ComputorOperand {valid, .. } if !valid => ComputorElem { unit: ComputorUnit::NONE },
			ComputorOperand {is_vect, .. } if is_vect.0 || is_vect.1 =>
				ComputorElem::vec_to_vec_operation(self, other, elems_info, ComputorAction::SUB),
			ComputorOperand {is_img, .. } if is_img.0 != is_img.1 =>
				ComputorElem { unit: ComputorUnit::VECT( vec![self, ComputorElem{ unit: ComputorUnit::ATT("-".to_string()) } , other] ) },
			ComputorOperand {is_float, is_img, .. } if is_float.0 || is_float.1 =>
				ComputorElem { unit: ComputorUnit::F64(varf.0 - varf.1, is_img.0) },
			ComputorOperand {is_float, is_img, .. } if !is_float.0 && !is_float.1 =>
				ComputorElem { unit: ComputorUnit::I64(vari.0 - vari.1, is_img.0) },
			_ => ComputorElem { unit: ComputorUnit::NONE }
		}
	}
}

// TODO: DO POW()

impl Mul for ComputorElem {
	type Output = ComputorElem;

	fn mul(self, other: ComputorElem) -> ComputorElem {
		let mut varf = (0.0, 0.0);
		let mut vari = (0, 0);
		
		let elems_info = self.get_type_vars(&other, &mut varf, &mut vari);
		// imaginari mull
		match elems_info {
			ComputorOperand {valid, .. } if !valid => 
				ComputorElem { unit: ComputorUnit::NONE },
			ComputorOperand {is_vect, .. } if is_vect.0 || is_vect.1 => 
				ComputorElem::vec_to_vec_operation(self, other, elems_info, ComputorAction::MUL),
			// ComputorOperand {is_img, same_pow, .. } if same_pow &&  =>
				// ComputorElem { unit: ComputorUnit::VECT( vec![self, ComputorElem{ unit: ComputorUnit::ATT("*".to_string()) } , other] ) },
			ComputorOperand {is_float, is_img, .. } if is_float.0 || is_float.1 => {
					ComputorElem { unit: ComputorUnit::F64(varf.0 * varf.1, if is_img.0 || is_img.1 { true } else { false }) }
			},
			ComputorOperand {is_float, is_img, .. } if !is_float.0 && !is_float.1 => {
				ComputorElem { unit: ComputorUnit::I64(vari.0 * vari.1, if is_img.0 || is_img.1 { true } else { false }) }
			},
			_ => ComputorElem { unit: ComputorUnit::NONE }
		}
	}
}

impl Div for ComputorElem {
	type Output = ComputorElem;

	fn div(self, other: ComputorElem) -> ComputorElem {
		let mut varf = (0.0, 0.0);
		let mut vari = (0, 0);
		
		let elems_info = self.get_type_vars(&other, &mut varf, &mut vari);
		// imaginari Div
		//TODO: ADD MODULO TO CHECK IF REST IS INT
		match elems_info {
			ComputorOperand {valid, .. } if !valid => ComputorElem { unit: ComputorUnit::NONE },
			ComputorOperand {is_vect, .. } if is_vect.0 || is_vect.1 => 
				ComputorElem::vec_to_vec_operation(self, other, elems_info, ComputorAction::DIV),
			ComputorOperand {is_float, is_img, .. } if (is_float.0 || is_float.1) && varf.1 != 0.0 => 
				ComputorElem { unit: ComputorUnit::F64(varf.0 / varf.1, if is_img.0 || is_img.1 { true } else { false }) },
			ComputorOperand {is_float, is_img, .. } if (!is_float.0 && !is_float.1) && vari.1 != 0 => 
				ComputorElem { unit: ComputorUnit::I64(vari.0 / vari.1, if is_img.0 || is_img.1 { true } else { false }) },
			_ => ComputorElem { unit: ComputorUnit::NONE }
		}
	}
}

// impl BitXor for ComputorElem {
// 	type Output = ComputorElem;

// 	fn bitxor(self, other: ComputorElem) -> ComputorElem {
// 		let mut varf = (0.0, 0.0);
// 		let mut vari = (0, 0);
		
// 		let elems_info = self.get_type_vars(&other, &mut varf, &mut vari);
// 		match elems_info {
// 			ComputorOperand {valid, .. } if !valid => ComputorElem { unit: ComputorUnit::NONE },
// 			ComputorOperand {is_vect, .. } if is_vect.0 || is_vect.1 => 
// 				ComputorElem::vec_to_vec_operation(self, other, elems_info, ComputorAction::DIV),
// 			ComputorOperand {is_float, is_img, .. } if (is_float.0 || is_float.1) && varf.1 != 0.0 => 
// 				ComputorElem { unit: ComputorUnit::F64(varf.0.powf(varf.1) , if is_img.0 || is_img.1 { true } else { false }) },
// 			ComputorOperand {is_float, is_img, .. } if (!is_float.0 && !is_float.1) && vari.1 != 0 => 
// 				ComputorElem { unit: ComputorUnit::I64(vari.0.pow(vari.1), if is_img.0 || is_img.1 { true } else { false }) },
// 			_ => ComputorElem { unit: ComputorUnit::NONE }
// 		}
// 	}
// }

// impl Rem for ComputorElem {
// 	type Output = ComputorElem;

// 	fn rem(self, other: ComputorElem) -> ComputorElem {
// 		let mut varf = (0.0, 0.0);
// 		let mut vari = (0, 0);
		
// 		let elems_info = self.get_type_vars(&other, &mut varf, &mut vari);
// 		match elems_info {
// 			ComputorOperand {valid, .. } if !valid => ComputorElem { unit: ComputorUnit::NONE },
// 			ComputorOperand {is_vect, .. } if is_vect.0 || is_vect.1 => 
// 				ComputorElem::vec_to_vec_operation(self, other, elems_info, ComputorAction::DIV),
// 			ComputorOperand {is_float, is_img, .. } if (is_float.0 || is_float.1) && varf.1 != 0.0 => 
// 				ComputorElem { unit: ComputorUnit::F64(varf.0 % varf.1 , if is_img.0 || is_img.1 { true } else { false }) },
// 			ComputorOperand {is_float, is_img, .. } if (!is_float.0 && !is_float.1) && vari.1 != 0 => 
// 				ComputorElem { unit: ComputorUnit::I64(vari.0 % (vari.1, if is_img.0 || is_img.1 { true } else { false }) },
// 			_ => ComputorElem { unit: ComputorUnit::NONE }
// 		}
// 	}
// }
