use crate::runtime::funct::Funct;

#[derive(Debug)]
pub struct Queue(pub Vec<Funct>);

impl Queue {
	pub fn call(&self, args: Queue) -> Funct {
		Funct::error("queue call stub")
	}
}
