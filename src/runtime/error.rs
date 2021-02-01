use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::queue::Queue;
use crate::runtime::text::Text;

#[derive(Debug)]
pub struct Error {
	pub msg: Text,
	pub wrap: Option<Rc<Error>>,
}

impl Error {
	pub fn new<T: Into<Text>>(msg: T) -> Rc<Self> {
		Rc::new(Self {
			msg: msg.into(),
			wrap: None,
		})
	}

	pub fn wrap<T: Into<Text>>(self: Rc<Self>, msg: T) -> Rc<Self> {
		Rc::new(Self {
			msg: msg.into(),
			wrap: Some(self),
		})
	}

	pub fn call(self: Rc<Self>, args: Rc<Queue>) -> Funct {
		Funct::Error(self.wrap(format!("called with args {:?}", args)))
	}
}
