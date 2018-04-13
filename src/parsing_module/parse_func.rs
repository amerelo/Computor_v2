use parsing_module::parse::{ get_var };
use elemt_module::computorelem::{ComputorUnit, ComputorElem};

named!(pub get_func<&str, ComputorElem>, do_parse!(
	name: ws!(get_var) >>
	tag!("(") >>
	var: ws!(get_var) >>
	ws!(tag!(")")) >>
	ws!(tag!("=")) >>
	(ComputorElem{ unit: ComputorUnit::FUNC(name.var_to_string() , var.var_to_string()) })
));
