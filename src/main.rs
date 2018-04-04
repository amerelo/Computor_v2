#[macro_use]

extern crate nom;
extern crate tui;
extern crate termion;

mod parsing_module;
mod elemt_module;

use std::{sync::mpsc, collections::HashMap, io, thread };
use termion::{ event, input::TermRead };
use tui::{ Terminal, backend::MouseBackend, 
	widgets::{ Block, Borders, Item, List, Paragraph, Widget }, 
	layout::{ Direction, Group, Rect, Size }, 
	style::{ Color, Style }
};
use parsing_module::parse::{ select_next_parse, get_var, select_parser}; //, expr
use elemt_module::computorelem::{ComputorUnit, ComputorElem};
// use parsing::parse_matrix::{ matrix };
// use std::num::ParseIntError;

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

fn computorelem_to_string(computorelems: Vec<ComputorElem>) -> String
{
	let mut newline: String = String::new();

	// newline = computorelems.iter().fold(String::new(),|mut acc, var| acc.push_str( &var.var_so_strong() )  );
	for elem in computorelems {
		newline.push_str(&elem.var_to_string());
		newline.push(' ');
	}
	newline
}

// fn select_next_parse(elem_list: &Vec<ComputorElem>)
// {
	// if elem_list.len() < 2 {
	// 	println!("Erorr");
	// 	return ;
	// }

	// if let ComputorUnit::VAR(ref var) = elem_list[0].unit {
	// 	if let ComputorUnit::ATT(ref att) = elem_list[1].unit {
	// 		if att == "=" {
	// 			println!("new var is |{}|", var);
	// 		}
	// 	}	
	// }
// }

// pub fn dump<T: Debug>(res: nom::IResult<&str,T>)
fn dump(res: nom::IResult<&str, Vec<ComputorElem>>, var_list: &mut HashMap<String, ComputorElem>)
{
	if let nom::IResult::Done(rest, elems) = res {
		if !rest.is_empty() {
			println!("invalid command > {:?}", rest)
		} else {
			// replace var
			println!("{:?}", select_next_parse(&computorelem_to_string(elems)));
		}
	} else {
		println!("Bad Format");
	}
}

fn pars_entry(var_list: &mut HashMap<String, ComputorElem>) {
	let mut line: String = String::new();

	loop {
		std::io::stdin().read_line(&mut line).ok().expect("Failed to read line");
		// test_parse
		dump(select_parser(&mut line), var_list);
		// println!("{:?}", expr(&mut line));
		// test_nom(&mut line);
		line.clear();
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
