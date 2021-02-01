use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::queue::Queue;
use crate::runtime::under::Under;

pub fn call_fn(_under: Rc<Under>, args: Rc<Queue>) -> Funct {
	match args.0.borrow().len() {
		0 => Funct::calc(0),
		1 => Funct::error("calc construct stub"),
		_ => Funct::error("calc construct too many args"),
	}
}
