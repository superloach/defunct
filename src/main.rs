use std::env;
use std::io::{stdin, Read};

mod interp;
mod parser;
mod runtime;

use interp::Interp;
use runtime::block::Block;
use runtime::funct::Funct;
use runtime::lib::Lib;
use runtime::native::Native;
use runtime::queue::Queue;
use runtime::text::Text;
use runtime::under::Under;

fn main() {
	let stdlib = Lib::new()
		.add("foo", Funct::text("bar"))
		.add_native(Native::new("print", &|_under, args| {
			println!("print {:#?}", args);
			Funct::Zilch
		}));

	let under = Under::new(stdlib);
	dbg!(&under);

	let mut code = String::new();

	match stdin().read_to_string(&mut code) {
		Ok(_) => {
			dbg!(&code);

			let mut p = parser::Parser::new(&code);

			match p.parse() {
				Ok(block) => {
					dbg!(&block);

					let prgm = Block::interp(block);
					dbg!(&prgm);

					let mut args = Queue(Vec::new());
					for arg in env::args().skip(1) {
						args.0.push(Funct::text(arg));
					}
					dbg!(&args);

					let result = prgm.call(under, args);
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
