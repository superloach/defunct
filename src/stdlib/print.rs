use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::queue::Queue;
use crate::runtime::under::Under;

pub fn call_fn(_under: Rc<Under>, args: Rc<Queue>) -> Funct {
	dbg!(args);
	Funct::Zilch
}
