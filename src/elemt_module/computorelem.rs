use std::ops::{ Add, Sub ,Div , Mul};
use std::fmt::{ Display, Formatter, Result};

#[derive(PartialEq, Debug, Clone)] 
pub enum ComputorUnit {
	I64(i64, bool),
	F64(f64, bool),
	VAR(String),
	ATT(String),
	MAT(Vec<Vec<ComputorElem>>),
	VECT(Vec<ComputorElem>),
	FUNC(String, String),
	NEWVAR(String),
	SHOW,
	NONE
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

	// pub fn mult_elem_vec(&self, other: Vec<ComputorElem>)
	// {
	// 	for 
	// }
	
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
		else if let ComputorUnit::I64(val, img) = other.unit { vari.1 = val; eleminfo.is_float.0 = false; eleminfo.is_img.1 = img}
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
				if img == true {
					return write!(f, "{}i ", var.to_string());
				} else {
					return write!(f, "{} ", var.to_string());
				}
			},
			ComputorUnit::F64(var, img) => {
				if img == true {
					return write!(f, "{}i ", var.to_string());
				} else {
					return write!(f, "{} ", var.to_string());
				}
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

		// if let ComputorUnit::VECT(elems) = self.unit {
		// 	for elem in elems.iter_mut() {
		// 		if let ComputorUnit::I64(val, img) = elem.unit { if img {elem + other} }
		// 	}

		// 	return ComputorElem { unit: ComputorUnit::F64(42.42, false ) }
		// }
		// if let ComputorUnit::VECT(ref elems) = other.unit { println!("2 ----------- {:#?}", elems)}
		let elems_info = self.get_type_vars(&other, &mut varf, &mut vari);
		match elems_info {
			ComputorOperand {valid, .. } if !valid => ComputorElem { unit: ComputorUnit::NONE },
			ComputorOperand {is_vect, .. } if is_vect.0 || is_vect.1 => ComputorElem { unit: ComputorUnit::NONE },
			ComputorOperand {is_img, .. } if is_img.0 || is_img.1 => ComputorElem { unit: ComputorUnit::VECT( vec![self, ComputorElem{ unit: ComputorUnit::ATT("+".to_string()) } , other] ) },
			ComputorOperand {is_float, .. } if is_float.0 || is_float.1 => ComputorElem { unit: ComputorUnit::F64(varf.0 + varf.1, false ) },
			ComputorOperand {is_float, .. } if !is_float.0 && !is_float.1 => ComputorElem { unit: ComputorUnit::I64(vari.0 + vari.1, false ) },
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
			ComputorOperand {is_img, .. } if is_img.0 || is_img.1 => ComputorElem { unit: ComputorUnit::VECT( vec![self, ComputorElem{ unit: ComputorUnit::ATT("-".to_string()) } , other] ) },
			ComputorOperand {is_float, .. } if is_float.0 || is_float.1 => ComputorElem { unit: ComputorUnit::F64(varf.0 - varf.1, false ) },
			ComputorOperand {is_float, .. } if !is_float.0 && !is_float.1 => ComputorElem { unit: ComputorUnit::I64(vari.0 - vari.1, false ) },
			_ => ComputorElem { unit: ComputorUnit::NONE }
		}
	}
}

impl Mul for ComputorElem {
	type Output = ComputorElem;

	fn mul(self, other: ComputorElem) -> ComputorElem {
		let mut varf = (0.0, 0.0);
		let mut vari = (0, 0);
		
		let elems_info = self.get_type_vars(&other, &mut varf, &mut vari);
		match elems_info {
			ComputorOperand {valid, .. } if !valid => ComputorElem { unit: ComputorUnit::NONE },
			ComputorOperand {is_float, .. } if is_float.0 || is_float.1 => ComputorElem { unit: ComputorUnit::F64(varf.0 * varf.1, false ) },
			ComputorOperand {is_float, .. } if !is_float.0 && !is_float.1 => ComputorElem { unit: ComputorUnit::I64(vari.0 * vari.1, false ) },
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
		//TODO: ADD MODULO TO CHECK IF REST IS INT
		match elems_info {
			ComputorOperand {valid, .. } if !valid => ComputorElem { unit: ComputorUnit::NONE },
			ComputorOperand {is_float, .. } if (is_float.0 || is_float.1) && varf.1 != 0.0 => ComputorElem { unit: ComputorUnit::F64(varf.0 / varf.1, false ) },
			ComputorOperand {is_float, .. } if (!is_float.0 && !is_float.1) && vari.1 != 0 => ComputorElem { unit: ComputorUnit::I64(vari.0 / vari.1, false ) },
			_ => ComputorElem { unit: ComputorUnit::NONE }
		}
	}
}