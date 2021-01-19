use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::native::Native;
use crate::runtime::queue::Queue;
use crate::runtime::text::Text;

#[derive(Debug)]
pub struct Lib(pub RefCell<HashMap<Text, Funct>>);

impl Lib {
	pub fn new() -> Rc<Self> {
		Rc::new(Self(RefCell::new(HashMap::new())))
	}

	pub fn add<T: Into<Text>>(self: Rc<Self>, n: T, v: Funct) -> Rc<Self> {
		{
			let mut hm = self.0.borrow_mut();
			hm.insert(n.into(), v);
		}
		self
	}

	pub fn add_native(self: Rc<Self>, v: Rc<Native>) -> Rc<Self> {
		self.clone().add(v.name, Funct::Native(v));
		self
	}

	pub fn call(self: Rc<Self>, args: Queue) -> Funct {
		Funct::error("lib call stub")
	}
}
