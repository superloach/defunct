use std::env;
use std::io::stdin;

mod interp;
mod parser;
mod runtime;
mod stdlib;

use interp::Interp;
use parser::Parser;
use runtime::block::Block;
use runtime::funct::Funct;
use runtime::queue::Queue;
use runtime::under::Under;

fn main() {
	let std = stdlib::new();

	let args = Queue::new();
	for arg in env::args().skip(1) {
		args.0.borrow_mut().push(Funct::text(arg));
	}

	let under = Under::new(std, args);
	dbg!(&under);

	loop {
		let mut code = String::new();
		match stdin().read_line(&mut code) {
			Ok(0) => {
				dbg!("eof");
				return;
			}
			Ok(_) => {
				dbg!(&code);

				let mut p = Parser::new(&code);

				match p.parse_block() {
					Ok(block) => {
						dbg!(&block);

						let prgm = Block::interp(block);
						dbg!(&prgm);

						let result = prgm.call(under.clone());
						dbg!(&result);
					}
					Err(error) => {
						println!("{}", code);
						println!("{}^", " ".repeat(p.index));

						panic!("error: {:#?}", error)
					}
				}
			}
			Err(e) => panic!("reading code: {:#?}", e),
		}
	}
}
