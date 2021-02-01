use std::cell::RefCell;
use std::rc::Rc;

use crate::runtime::funct::Funct;

#[derive(Debug)]
pub struct Queue(pub RefCell<Vec<Funct>>);

impl Queue {
	pub fn new() -> Rc<Self> {
		Self::from(Vec::new())
	}

	pub fn from(v: Vec<Funct>) -> Rc<Self> {
		Rc::new(Self(RefCell::new(v)))
	}

	pub fn call(self: Rc<Self>, _args: Rc<Queue>) -> Funct {
		Funct::error("queue call stub")
	}
}
