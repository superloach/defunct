use std::rc::Rc;

use crate::runtime::lib::Lib;

pub mod calc;
pub mod doad;
pub mod print;

pub fn new() -> Rc<Lib> {
	Lib::new()
		.with_native("print", &print::call_fn)
		.with_native("doad", &doad::call_fn)
		.with_native("calc", &calc::call_fn)
}
