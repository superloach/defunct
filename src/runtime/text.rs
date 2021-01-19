use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::queue::Queue;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Text(pub String);

impl Text {
	pub fn new<T: Into<Self>>(s: T) -> Rc<Self> {
		Rc::new(s.into())
	}

	pub fn call(&self, args: Queue) -> Funct {
		let mut s = self.0.clone();
		for arg in args.0 {
			match s.find('$') {
				Some(offset) => s.replace_range(offset..(offset + 1), &arg.repr().0),
				None => s.push_str(&format!("[extra {}]", arg.repr().0)),
			}
			dbg!(&s);
		}
		Funct::text(s)
	}
}

impl<T> From<T> for Text
where
	String: From<T>,
{
	fn from(v: T) -> Self {
		Self(v.into())
	}
}
