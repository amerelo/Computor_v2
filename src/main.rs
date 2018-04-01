#[macro_use]
extern crate nom;

mod parsing;

use parsing::parse::test_nom;
// use std::num::ParseIntError;

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
