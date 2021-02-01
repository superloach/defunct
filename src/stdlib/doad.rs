use std::rc::Rc;

use crate::runtime::funct::Funct;
use crate::runtime::queue::Queue;
use crate::runtime::under::Under;

pub fn call_fn(_under: Rc<Under>, _args: Rc<Queue>) -> Funct {
	Funct::error("not a typewriter")
}
