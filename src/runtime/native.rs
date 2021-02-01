use std::fmt;
use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::queue::Queue;
use crate::runtime::under::Under;

pub type CallFn = dyn Fn(Rc<Under>, Rc<Queue>) -> Funct;

pub struct Native {
	pub name: &'static str,
	pub call_fn: &'static CallFn,
}

impl Native {
	pub fn new(name: &'static str, call_fn: &'static CallFn) -> Rc<Self> {
		Rc::new(Self { name, call_fn })
	}

	pub fn call(self: Rc<Self>, under: Rc<Under>, args: Rc<Queue>) -> Funct {
		(self.call_fn)(under, args)
	}
}

impl fmt::Debug for Native {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Native")
			.field("name", &self.name)
			.field("call_fn", &(self.call_fn as *const CallFn))
			.finish()
	}
}
