use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::lib::Lib;
use crate::runtime::queue::Queue;
use crate::runtime::text::Text;

#[derive(Debug)]
pub struct Under {
	pub args: Option<Queue>,
	pub usr: Rc<Lib>,
	pub std: Rc<Lib>,
	pub parent: Option<Rc<Under>>,
}

impl Under {
	pub fn new(std: Rc<Lib>) -> Rc<Self> {
		Rc::new(Self {
			args: None,
			usr: Lib::new(),
			std,
			parent: None,
		})
	}

	pub fn child(self: Rc<Self>, args: Queue) -> Rc<Self> {
		Rc::new(Self {
			args: Some(args),
			usr: Lib::new(),
			std: self.std.clone(),
			parent: Some(self.clone()),
		})
	}

	pub fn call(self: Rc<Self>, args: Queue) -> Funct {
		match args.0.len() {
			0 => Funct::Zilch,
			1 => match args.0[0].clone() {
				Funct::Text(t) => self.get(t),
				_ => Funct::error("under get: name not text"),
			},
			2 => match args.0[0].clone() {
				Funct::Text(t) => self.set(t.clone(), args.0[1].clone()),
				_ => Funct::error("under set: name not text"),
			},
			_ => Funct::error("under: too many args"),
		}
	}

	pub fn get(self: Rc<Self>, name: Rc<Text>) -> Funct {
		match name.0.chars().nth(0) {
			Some('@') => {
				let stdn: String = name.0.chars().skip(1).collect();
				let std = self.std.0.borrow();
				match std.get(&Text::from(&stdn)) {
					Some(f) => f.clone(),
					None => Funct::error(format!("no such std {}", &stdn)),
				}
			}
			_ => {
				let usr = self.usr.0.borrow();
				match usr.get(&name) {
					Some(f) => f.clone(),
					None => Funct::error(format!("no such usr {}", name.0)),
				}
			}
		}
	}

	pub fn set(self: Rc<Self>, name: Rc<Text>, value: Funct) -> Funct {
		Funct::error("under set stub")
	}
}
