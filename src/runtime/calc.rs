use std::rc::Rc;

use fraction::Decimal;

use crate::runtime::funct::Funct;
use crate::runtime::queue::Queue;

#[derive(Debug)]
pub struct Calc(pub Decimal);

impl<T> From<T> for Calc
where
	Decimal: From<T>,
{
	fn from(v: T) -> Self {
		Self(v.into())
	}
}

impl Calc {
	pub fn new<T: Into<Self>>(v: T) -> Rc<Self> {
		Rc::new(v.into())
	}

	pub fn call(self: Rc<Self>, args: Queue) -> Funct {
		Funct::error("calc call stub")
	}
}
