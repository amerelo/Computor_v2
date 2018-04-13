#![allow(dead_code)] // TODO: rm in the end

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
use parsing_module::parse::{ atribut_var, get_var, parser_elems}; //, expr
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

// return Option
fn computorelem_to_string(computorelems: &Vec<ComputorElem>, var_list: &mut HashMap<String, ComputorElem>) -> String
{
	let mut newline: String = String::new();

	// newline = computorelems.iter().fold(String::new(),|mut acc, var| acc.push_str( &var.var_so_strong() )  );
	for elem in computorelems {
		if let ComputorUnit::VAR(var) = elem.unit.clone() {
			match var_list.get(&var.to_lowercase()) {
				Some(val) => newline.push_str(&val.clone().var_to_string().to_lowercase()),
				None => println!("{} is not a variable yet ", &var)
			}
		} else {
			newline.push_str(&elem.clone().var_to_string().to_lowercase());
		}
		newline.push(' ');
	}
	newline
}

fn check_elem_parsed(res: nom::IResult<&str, Vec<ComputorElem>>) -> Result<Vec<ComputorElem>, io::Error>
{
	if let nom::IResult::Done(rest, elems) = res.clone() {
		if rest.is_empty() {
			return Ok(elems);
		} 
	}
	return Err(std::io::Error::new(std::io::ErrorKind::Other, "Bad format"));
}

fn set_var(var: String, elems: &Vec<ComputorElem>, var_list: &mut HashMap<String, ComputorElem>) -> bool
{
	if elems.len() == 1 && elems[0].unit != ComputorUnit::NONE {
		var_list.insert(var.clone().to_lowercase(), elems[0].clone());
		return true;
	}
	return false;
}

fn replace_var_for(new_var: String, old_var: String, vec: &mut Vec<ComputorElem>) -> String
{
	let mut newline: String = String::new();

	for mut elem in vec.iter_mut() {
		if elem.clone().var_to_string() == old_var {
			elem.unit = ComputorUnit::VAR(new_var.clone());
		}
	}
	for elem in vec.iter() {
		newline.push_str(&elem.clone().var_to_string().to_lowercase());
		newline.push(' ');
	}
	newline
}

fn new_func(computorelems: &Vec<ComputorElem>, var_list: &mut HashMap<String, ComputorElem>) -> bool
{
	if let ComputorUnit::FUNC(name, var) = computorelems[0].unit.clone() {
		let mut new_vec: Vec<_> = computorelems.clone();
		let mut new_vec: Vec<_> = new_vec.drain(1..).collect();

		let new_str = replace_var_for("42".to_owned(), var, &mut new_vec);
		if let Ok(_elems) = check_elem_parsed(atribut_var(&new_str)) {
			var_list.insert(name.to_lowercase(), ComputorElem{ unit: ComputorUnit::FUNCVAR(new_vec) } );
			return true;
		}
	}
	return false;
}

fn new_var(computorelems: &Vec<ComputorElem>, var_list: &mut HashMap<String, ComputorElem>) -> bool
{	
	if let ComputorUnit::NEWVAR(var) = computorelems[0].unit.clone() {
		let mut new_vec: Vec<_> = computorelems.clone();
		let mut new_vec: Vec<_> = new_vec.drain(1..).collect();

		let new_str = computorelem_to_string(&new_vec, var_list);
		if let Ok(elems) = check_elem_parsed(atribut_var(&new_str)) {
			return set_var(var, &elems, var_list);
		}
	}
	return false;
}

fn show_result(computorelems: &Vec<ComputorElem>, var_list: &mut HashMap<String, ComputorElem>) -> bool
{
	let mut new_vec: Vec<_> = computorelems.clone();
	
	if ComputorUnit::SHOW == new_vec[new_vec.len() - 1].unit {
		new_vec.pop();
		let new_str = computorelem_to_string(&new_vec, var_list);
		if let Ok(elems) = check_elem_parsed(atribut_var(&new_str)) {
			println!("elems> {:?}", elems);
			return true;
		}
	}
	return false;
}

fn identify_elements(computorelems: &Vec<ComputorElem>, var_list: &mut HashMap<String, ComputorElem>)
{
	if computorelems.len() < 2 {
		println!("Bad Format :( (need more details)");
		return ;
	}

	if !show_result(&computorelems, var_list) && !new_var(&computorelems, var_list) && !new_func(&computorelems, var_list) {
		println!("{}", "error in format");
	}
}

fn pars_entry(var_list: &mut HashMap<String, ComputorElem>) {
	let mut line: String = String::new();

	loop {
		std::io::stdin().read_line(&mut line).ok().expect("Failed to read line :)");
		if line.trim().is_empty() {
			continue;
		}

		if let Ok(elems) = check_elem_parsed(parser_elems(&mut line)) {
			identify_elements(&elems, var_list);
		}
		println!("{:?}", var_list);
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

	// Terminal initialization
	// let backend = MouseBackend::new().unwrap();
	// let mut terminal = Terminal::new(backend).unwrap();

	// Channels
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

	// App
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
