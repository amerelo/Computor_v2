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
use parsing_module::parse::{ atribut_var, parser_elems, vectorised}; //, expr
use elemt_module::computorelem::{ComputorUnit, ComputorElem};
// use parsing::parse_matrix::{ matrix };
// use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
struct ComputorLists {
	var_list: HashMap<String, ComputorElem>,
	func_list: HashMap<String, Vec<ComputorElem> >,
}

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
fn computorelem_to_string(computorelems: &Vec<ComputorElem>, computorlists: &mut ComputorLists) -> String
{
	let mut newline: String = String::new();

	// newline = computorelems.iter().fold(String::new(),|mut acc, var| acc.push_str( &var.var_so_strong() ));
	for elem in computorelems {
		if let ComputorUnit::VAR(var) = elem.unit.clone() {
			match computorlists.var_list.get(&var.to_lowercase()) {
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

// TODO: need to add (i, matrix)
fn set_var(var: String, elems: &Vec<ComputorElem>, computorlists: &mut ComputorLists) -> bool
{
	if elems.len() == 1 && elems[0].unit != ComputorUnit::NONE && var.to_lowercase() != "i" {
		computorlists.var_list.insert(var.clone().to_lowercase(), elems[0].clone());
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

// fn test_recursion(computorelems: &Vec<ComputorElem>)
// {
	
// }

fn test_op(computorelems: &mut Vec<ComputorElem>) -> Vec<ComputorElem>
{
	println!("VEC -> {:?}", computorelems);

	return computorelems.clone();
}

fn manage_imaginari(computorelems: &mut Vec<ComputorElem>, computorlists: &mut ComputorLists) -> bool
{
	if let ComputorUnit::NEWVAR(var) = computorelems[0].unit.clone() {
		let mut new_vec: Vec<_> = computorelems.drain(1..).collect();

		let new_str = computorelem_to_string(&new_vec, computorlists);
		
		if let Ok(mut elems) = check_elem_parsed(vectorised(&new_str) ) {
			test_op(&mut elems);
			// return set_var(var, &elems, computorlists);
			return true;
		} else {
			println!("Error at vectorice str -> {}", new_str);
		}
	}
	return false;
}

fn new_func(computorelems: &mut Vec<ComputorElem>, computorlists: &mut ComputorLists) -> bool
{
	if let ComputorUnit::FUNC(name, var) = computorelems[0].unit.clone() {
		let mut new_vec: Vec<_> = computorelems.drain(1..).collect();

		let new_str = replace_var_for("42".to_owned(), var, &mut new_vec);
		if let Ok(_elems) = check_elem_parsed(atribut_var(&new_str)) {
			computorlists.func_list.insert(name.to_lowercase(), new_vec);
			return true;
		}
	}
	return false;
}

fn new_var(computorelems: &mut Vec<ComputorElem>, computorlists: &mut ComputorLists) -> bool
{	
	if let ComputorUnit::NEWVAR(var) = computorelems[0].unit.clone() {
		let mut new_vec: Vec<_> = computorelems.drain(1..).collect();

		let new_str = computorelem_to_string(&new_vec, computorlists);
		if let Ok(elems) = check_elem_parsed(atribut_var(&new_str)) {
			return set_var(var, &elems, computorlists);
		}
	}
	return false;
}

fn show_result(computorelems: &Vec<ComputorElem>, computorlists: &mut ComputorLists) -> bool
{
	let mut new_vec: Vec<_> = computorelems.clone();
	
	if ComputorUnit::SHOW == new_vec[new_vec.len() - 1].unit {
		new_vec.pop();
		let new_str = computorelem_to_string(&new_vec, computorlists);
		if let Ok(elems) = check_elem_parsed(atribut_var(&new_str)) {
			//TODO: need to make print for show var
			println!("SHOW --> {:?}", elems);
			return true;
		}
	}
	return false;
}

// TODO: make verif of matrix, function, imaginari && etc..
fn identify_elements(computorelems: &mut Vec<ComputorElem>, computorlists: &mut ComputorLists)
{
	if computorelems.len() < 2 {
		println!("Bad Format :( (need more details)");
		return ;
	}

	manage_imaginari(computorelems, computorlists);
	// if !show_result(computorelems, computorlists) && !new_var(computorelems, computorlists) && !new_func(computorelems, computorlists) {
	// 	println!("{}", "error in format");
	// }
}

fn pars_entry(computorlists: &mut ComputorLists) {
	let mut line: String = String::new();

	loop {
		std::io::stdin().read_line(&mut line).ok().expect("Failed to read line :)");
		if line.trim().is_empty() {
			continue;
		}

		if let Ok(mut elems) = check_elem_parsed(parser_elems(&mut line)) {
			identify_elements(&mut elems, computorlists);
		}
		println!("VARS {:?}", computorlists.var_list);
		println!("FUNCS {:?}", computorlists.func_list);
		line.clear();
	}
}

fn main()
{
	// ############################################################
	// let mut computorlists: Vec<ComputorVar> = Vec::new();
	let mut computorlists: ComputorLists =  ComputorLists {
		func_list: HashMap::new(),
		var_list: HashMap::new()
	}; 

	println!("Welcome to computor_v2");
	pars_entry(&mut computorlists);
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
