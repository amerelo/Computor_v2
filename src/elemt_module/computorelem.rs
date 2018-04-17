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

#[derive(PartialEq, Debug, Clone)]
pub struct ComputorElem {
	pub unit: ComputorUnit,
}

impl ComputorElem {
	pub fn get_type_vars(self, other: &ComputorElem, is_int: &mut bool, f1: &mut f64, f2: &mut f64, i1: &mut i64, i2: &mut i64) -> bool
	{
		let set1: bool;
		let set2: bool;

		if let ComputorUnit::F64(val, _img) = self.unit { *f1 = val; set1 = true; } 
		else if let ComputorUnit::I64(val, _img) = self.unit { *i1 = val; set1 = false; } 
		else { return false }

		if let ComputorUnit::F64(val, _img) = other.unit { *f2 = val; set2 = true; } 
		else if let ComputorUnit::I64(val, _img) = other.unit { *i2 = val; set2 = false; } 
		else { return false }
		
		if set1 == true || set2 == true {
			if set1 == true && set2 == false { *f2 = *i2 as f64; }
			else if set1 == false && set2 == true { *f1 = *i1 as f64; }
			*is_int = false;
		} else {
			*is_int = true;
		}
		return true;
	}
}

impl Display for ComputorElem {
	fn fmt(&self, f: &mut Formatter) -> Result {
		
		match self.unit {
			ComputorUnit::I64(var, img) => {
				if img == true {
					return write!(f, "{} * i ", var.to_string());
				} else {
					return write!(f, "{} ", var.to_string());
				}
			},
			ComputorUnit::F64(var, img) => {
				if img == true {
					return write!(f, "{} * i ", var.to_string());
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
		let mut varf1: f64 = 0.0;
		let mut varf2: f64 = 0.0;
		let mut vari1: i64 = 0;
		let mut vari2: i64 = 0;
		let mut is_int: bool = false;
		
		if !self.get_type_vars(&other, &mut is_int, &mut varf1, &mut varf2, &mut vari1, &mut vari2) {
			return ComputorElem { unit: ComputorUnit::NONE };
		}

		if is_int {
			ComputorElem { unit: ComputorUnit::I64(vari1 + vari2, false ) }
		} else {
			ComputorElem { unit: ComputorUnit::F64(varf1 + varf2, false ) }
		}
	}
}

impl Sub for ComputorElem {
	type Output = ComputorElem;

	fn sub(self, other: ComputorElem) -> ComputorElem {
		let mut varf1: f64 = 0.0;
		let mut varf2: f64 = 0.0;
		let mut vari1: i64 = 0;
		let mut vari2: i64 = 0;
		let mut is_int: bool = false;
		
		if !self.get_type_vars(&other, &mut is_int, &mut varf1, &mut varf2, &mut vari1, &mut vari2) {
			return ComputorElem { unit: ComputorUnit::NONE };
		}

		if is_int {
			ComputorElem { unit: ComputorUnit::I64(vari1 - vari2, false ) }
		} else {
			ComputorElem { unit: ComputorUnit::F64(varf1 - varf2, false ) }
		}
	}
}

impl Mul for ComputorElem {
	type Output = ComputorElem;

	fn mul(self, other: ComputorElem) -> ComputorElem {
		let mut varf1: f64 = 0.0;
		let mut varf2: f64 = 0.0;
		let mut vari1: i64 = 0;
		let mut vari2: i64 = 0;
		let mut is_int: bool = false;
		
		if !self.get_type_vars(&other, &mut is_int, &mut varf1, &mut varf2, &mut vari1, &mut vari2) {
			return ComputorElem { unit: ComputorUnit::NONE };
		}

		if is_int {
			ComputorElem { unit: ComputorUnit::I64(vari1 * vari2, false ) }
		} else {
			ComputorElem { unit: ComputorUnit::F64(varf1 * varf2, false ) }
		}
	}
}

impl Div for ComputorElem {
	type Output = ComputorElem;

	fn div(self, other: ComputorElem) -> ComputorElem {
		let mut varf1: f64 = 0.0;
		let mut varf2: f64 = 0.0;
		let mut vari1: i64 = 0;
		let mut vari2: i64 = 0;
		let mut is_int: bool = false;
		
		if !self.get_type_vars(&other, &mut is_int, &mut varf1, &mut varf2, &mut vari1, &mut vari2) {
			return ComputorElem { unit: ComputorUnit::NONE };
		}

		if is_int {
			if varf2 == 0.0 { return ComputorElem { unit: ComputorUnit::NONE }; }
			ComputorElem { unit: ComputorUnit::I64(vari1 / vari2, false ) }
		} else {
			//TODO: ADD MODULO TO CHECK IF REST IS INT
			ComputorElem { unit: ComputorUnit::F64(varf1 / varf2, false ) }
		}
	}
}