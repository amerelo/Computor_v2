#[macro_use]

extern crate nom;
extern crate tui;
extern crate termion;

mod parsing;

use std::sync::mpsc;
use std::fmt::Debug;
use std::collections::HashMap;
use std::{io, thread};
use std::ops::{ Add, Sub ,Div , Mul};

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Item, List, Paragraph, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};

use parsing::parse::{ get_var, select_parser}; //, expr
// use parsing::parse_matrix::{ matrix };
// use std::num::ParseIntError;


#[derive(PartialEq, Debug)]
pub enum ComputorUnit {
	I64(i64),
	F64(f64),
	VAR(String),
	ATT(String),
	MAT(Vec<ComputorUnit>),
	NONE
}
// #################################################################################################
#[derive(PartialEq, Debug)]
pub struct ComputorElem {
	pub unit: ComputorUnit,
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

// #################################################################################################

struct App {
	size: Rect,
	input: String,
	messages: Vec<String>,
}

impl App {
	fn new() -> App {
		App {
			size: Rect::default(),
			input: String::new(),
			messages: Vec::new(),
		}
	}
}

enum Event {
	Input(event::Key),
}

// fn get_var(elems: Vec<ComputorElem>, var_list: &mut HashMap<String, ComputorElem> ) {

// map.insert(field_name, field_value);
// }

// TEST FUNCTION
// pub fn test_nom(name : &mut String)
// {
//
// 	println!("{:?}", select_parser(name));

	// println!("{:?}", get_var(name));
	// dump(expr(name));
	// dump(matrix(name));
// }

// TEST FUNCTION
// rest.drain(..());

// pub fn dump<T: Debug>(res: nom::IResult<&str,T>)
pub fn dump(res: nom::IResult<&str, Vec<ComputorElem>>)
{
	if let nom::IResult::Done(rest, value) = res {
		if !rest.is_empty() {
			println!("invalid command> {:?}", rest)
		} else {
			// println!("{:?}", value);
			for elem in value.iter()
			{
				let tmp: i64 = 20;
				if let ComputorUnit::F64(val) = elem.unit {

					println!("var -> {:?}", val + tmp as f64);
				}
			}
		}
	} else {
		println!("ERROR");
	}

	// match res {
	// 		nom::IResult::Done(rest, value) => {
	// 			if !rest.is_empty() {
	// 				println!("invalid command> {:?}", rest)
	// 			} else {
	// 				// println!("Done {:?}", value)
	// 				// for elem in value.iter()
	// 				// {
	// 				// 	if let ComputorUnit::F64{val} = elem.unit {
	// 				// 		print!("{:?}", val);
	// 				// 	}
	// 				// }
	// 			}
	// 		},
	// 		nom::IResult::Incomplete(needed) => { println!("Needed {:?}", needed) },
	// 		nom::IResult::Error(err) => { println!("Err {:?}", err ) },
	// }
}

fn pars_entry(var_list: &mut HashMap<String, ComputorElem>) {
	let mut name = String::new();

	loop {
		std::io::stdin().read_line(&mut name).ok().expect("Failed to read line");
		// test_parse
		dump(select_parser(&mut name));
		// test_nom(&mut name);
		name.clear();
	}
}

fn main()
{
	// ############################################################
	// let mut var_list: Vec<ComputorVar> = Vec::new();
	let mut var_list: HashMap<String, ComputorElem> = HashMap::new();

	println!("Welcome to computor_v2");
	pars_entry(&mut var_list);
	// ############################################################

	// // Terminal initialization
	// let backend = MouseBackend::new().unwrap();
	// let mut terminal = Terminal::new(backend).unwrap();

	// // Channels
	// let (tx, rx) = mpsc::channel();
	// let input_tx = tx.clone();

	// // Input
	// thread::spawn(move || {
	// 	let stdin = io::stdin();
	// 	for c in stdin.keys() {
	// 		let evt = c.unwrap();
	// 		input_tx.send(Event::Input(evt)).unwrap();
	// 		if evt == event::Key::Esc {
	// 			break;
	// 		}
	// 	}
	// });

	// // App
	// let mut app = App::new();

	// // First draw call
	// terminal.clear().unwrap();
	// terminal.hide_cursor().unwrap();
	// app.size = terminal.size().unwrap();
	// draw(&mut terminal, &app);

	// loop {
	// 	let size = terminal.size().unwrap();
	// 	if app.size != size {
	// 		terminal.resize(size).unwrap();
	// 		app.size = size;
	// 	}

	// 	let evt = rx.recv().unwrap();
	// 	match evt {
	// 		Event::Input(input) => match input {
	// 			event::Key::Esc => {
	// 				break;
	// 			}
	// 			event::Key::Char('\n') => {
	// 				app.messages.push(app.input.drain(..).collect());
	// 			}
	// 			event::Key::Char(c) => {
	// 				app.input.push(c);
	// 			}
	// 			event::Key::Backspace => {
	// 				app.input.pop();
	// 			}
	// 			_ => {}
	// 		},
	// 	}
	// 	draw(&mut terminal, &app);
	// }

	// terminal.show_cursor().unwrap();
	// terminal.clear().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) {
	Group::default()
	.direction(Direction::Vertical)
	.margin(1)
	.sizes(&[Size::Fixed(3), Size::Min(1)])
	.render(t, &app.size, |t, chunks| {

		Paragraph::default()
		.style(Style::default().fg(Color::Yellow))
		.block(Block::default().borders(Borders::ALL).title("Input"))
		.text(&app.input)
		.render(t, &chunks[0]);

		List::new(
			app.messages
			.iter()
			.enumerate()
			.map(|(i, m)| Item::Data(format!("{}: {}", i, m))),
		).block(Block::default().borders(Borders::ALL).title("Messages"))
		.render(t, &chunks[1]);
	});

	t.draw().unwrap();
}
