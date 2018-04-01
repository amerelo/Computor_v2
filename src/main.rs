#[macro_use]
extern crate nom;

mod parsing;

use parsing::parse::{ dump, get_var, select_parser }; //, expr
// use parsing::parse_matrix::{ matrix };
// use std::num::ParseIntError;


// TEST FUNCTION
pub fn test_nom(name : &mut String)
{
	// println!("{:?}", get_var(name));
	println!("{:?}", select_parser(name));
	// dump();
	// dump(expr(name));
	// dump(matrix(name));
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
