extern crate extra;
use std::env::args;
use std::io::{self, Read, Write};
use extra::io::fail;

static HELP: &str = "???"



fn main()
{

	let args = args().skip(1);
	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	let stdout = io::stdout();
	let mut stdout = stdout.lock();
	let mut stderr = io::stderr();

	for i in args
	{
		match i.as_str()
		{
			"-h" | "-H" | "--h" | "--HELP" | "--help" =>
			{
				print!("{}", HELP);
			}
		
		_ => fail("[ERR] Unknown argument. For more information, refer to the wiki, or use the '--help' argument.", &mut stderr),
		}
	}

	loop
	{
		let mup input = [0];
		let _ = stdin.read(&mut input);

		match input[0]
		{
			b'k' => print!("\x1b[A"),
			b'j' => print!("\x1b[B"),
			b'l' => print!("\x1b[C"),
			b'h' => print!("\x1b[D"),
			b'q' => break,
			_ => {}
		}
		let _ = stdout.flush();
	}
}
