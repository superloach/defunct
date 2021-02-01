use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::lib::Lib;
use crate::runtime::queue::Queue;
use crate::runtime::text::Text;

#[derive(Debug)]
pub struct Under {
	pub args: Rc<Queue>,
	pub usr: Rc<Lib>,
	pub std: Rc<Lib>,
	pub parent: Option<Rc<Under>>,
}

impl Under {
	pub fn new(std: Rc<Lib>, args: Rc<Queue>) -> Rc<Self> {
		Rc::new(Self {
			args,
			usr: Lib::new(),
			std,
			parent: None,
		})
	}

	pub fn child(self: Rc<Self>, args: Rc<Queue>) -> Rc<Self> {
		Rc::new(Self {
			args,
			usr: Lib::new(),
			std: self.std.clone(),
			parent: Some(self),
		})
	}

	pub fn call(self: Rc<Self>, args: Rc<Queue>) -> Funct {
		let args = &*args.0.borrow();
		match args.len() {
			0 => Funct::Zilch,
			1 => match args[0].clone() {
				Funct::Text(t) => self.get(t),
				_ => Funct::error("under get: name not text"),
			},
			2 => match args[0].clone() {
				Funct::Text(t) => self.set(t, args[1].clone()),
				_ => Funct::error("under set: name not text"),
			},
			_ => Funct::error("under: too many args"),
		}
	}

	pub fn get(self: Rc<Self>, name: Rc<Text>) -> Funct {
		let mut chars = name.0.chars();
		match chars.next() {
			Some('@') => {
				let stdn: String = chars.collect();
				match &stdn as &str {
					"args" => Funct::Queue(self.args.clone()),
					"usr" => Funct::Lib(self.usr.clone()),
					"std" => Funct::Lib(self.std.clone()),
					"parent" => match &self.parent {
						Some(parent) => Funct::Under(parent.clone()),
						None => Funct::Zilch,
					},
					_ => {
						let std = self.std.0.borrow();
						match std.get(&Text::from(&stdn)) {
							Some(f) => f.clone(),
							None => Funct::error(format!("no such std {}", &stdn)),
						}
					}
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

	pub fn set(self: Rc<Self>, _name: Rc<Text>, _value: Funct) -> Funct {
		Funct::error("under set stub")
	}
}
