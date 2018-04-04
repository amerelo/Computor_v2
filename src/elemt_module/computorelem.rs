use std::ops::{ Add, Sub ,Div , Mul};

#[derive(PartialEq, Debug)]
pub enum ComputorUnit {
	I64(i64),
	F64(f64),
	VAR(String),
	ATT(String),
	MAT(Vec<ComputorUnit>),
	NONE
}

#[derive(PartialEq, Debug)]
pub struct ComputorElem {
	pub unit: ComputorUnit,
}

impl ComputorElem {
	// TODO: set error if NONE
	pub fn var_to_string(self) -> String
	{
		let mut mystring: String = String::new();
		match self.unit {
			ComputorUnit::I64(var) => mystring = var.to_string(),
			ComputorUnit::F64(var) => mystring = var.to_string(),
			ComputorUnit::VAR(var) => mystring = var,
			ComputorUnit::ATT(var) => mystring = var,
			ComputorUnit::MAT(var) => println!("TODO mat to string"),
			ComputorUnit::NONE => println!("ERROR NONE value"),
		}
		return mystring;
	}
}

impl Add for ComputorElem {
	type Output = ComputorElem;

	fn add(self, other: ComputorElem) -> ComputorElem {
		let mut varf1: f64 = 0.0;
		let mut varf2: f64 = 0.0;
		let mut vari1: i64 = 0;
		let mut vari2: i64 = 0;
		let set1: bool;
		let set2: bool;

		if let ComputorUnit::F64(val1) = self.unit { varf1 = val1;  set1 = true; } 
		else if let ComputorUnit::I64(val1) = self.unit { vari1 = val1; set1 = false; } 
		else { return  ComputorElem { unit: ComputorUnit::NONE }; }

		if let ComputorUnit::F64(val2) = other.unit { varf2 = val2;  set2 = true; } 
		else if let ComputorUnit::I64(val2) = other.unit { vari2 = val2; set2 = false; } 
		else { return ComputorElem { unit: ComputorUnit::NONE }; }
		
		if set1 == true || set2 == true {
			if set1 == true && set2 == false { varf2 = vari2 as f64; }
			else { varf1 = vari1 as f64; }
			ComputorElem { unit: ComputorUnit::F64(varf1 + varf2 ) }	
		}
		else {
			ComputorElem { unit: ComputorUnit::I64(vari1 + vari2 ) }	
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
		let set1: bool;
		let set2: bool;

		if let ComputorUnit::F64(val1) = self.unit { varf1 = val1;  set1 = true; } 
		else if let ComputorUnit::I64(val1) = self.unit { vari1 = val1; set1 = false; } 
		else { return  ComputorElem { unit: ComputorUnit::NONE }; }

		if let ComputorUnit::F64(val2) = other.unit { varf2 = val2;  set2 = true; } 
		else if let ComputorUnit::I64(val2) = other.unit { vari2 = val2; set2 = false; } 
		else { return ComputorElem { unit: ComputorUnit::NONE }; }
		
		if set1 == true || set2 == true {
			if set1 == true && set2 == false { varf2 = vari2 as f64; }
			else { varf1 = vari1 as f64; }
			ComputorElem { unit: ComputorUnit::F64(varf1 - varf2 ) }	
		}
		else {
			ComputorElem { unit: ComputorUnit::I64(vari1 - vari2 ) }	
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
		let set1: bool;
		let set2: bool;

		if let ComputorUnit::F64(val1) = self.unit { varf1 = val1;  set1 = true; } 
		else if let ComputorUnit::I64(val1) = self.unit { vari1 = val1; set1 = false; } 
		else { return  ComputorElem { unit: ComputorUnit::NONE }; }

		if let ComputorUnit::F64(val2) = other.unit { varf2 = val2;  set2 = true; } 
		else if let ComputorUnit::I64(val2) = other.unit { vari2 = val2; set2 = false; } 
		else { return ComputorElem { unit: ComputorUnit::NONE }; }
		
		if set1 == true || set2 == true {
			if set1 == true && set2 == false { varf2 = vari2 as f64; }
			else { varf1 = vari1 as f64; }
			ComputorElem { unit: ComputorUnit::F64(varf1 * varf2 ) }	
		}
		else {
			ComputorElem { unit: ComputorUnit::I64(vari1 * vari2 ) }	
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
		let set1: bool;
		let set2: bool;

		if let ComputorUnit::F64(val1) = self.unit { varf1 = val1;  set1 = true; } 
		else if let ComputorUnit::I64(val1) = self.unit { vari1 = val1; set1 = false; } 
		else { return  ComputorElem { unit: ComputorUnit::NONE }; }

		if let ComputorUnit::F64(val2) = other.unit { varf2 = val2;  set2 = true; } 
		else if let ComputorUnit::I64(val2) = other.unit { vari2 = val2; set2 = false; } 
		else { return ComputorElem { unit: ComputorUnit::NONE }; }
		
		if set1 == true || set2 == true {
			if set1 == true && set2 == false { varf2 = vari2 as f64; }
			else { varf1 = vari1 as f64; }
			if varf2 == 0.0 { return ComputorElem { unit: ComputorUnit::NONE }; }
			ComputorElem { unit: ComputorUnit::F64(varf1 / varf2 ) }	
		}
		else {
			if vari2 == 0 { return ComputorElem { unit: ComputorUnit::NONE }; }
			ComputorElem { unit: ComputorUnit::I64(vari1 / vari2 ) }	
		}
	}
}