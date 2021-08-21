use progbar::ProgressBar;
use std::str::from_utf8;
use tty::mv_csr_up;
use std::io::{Stdout, Result, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

pub struct MultiBar<T: Write>
{
	nline: usize,
	line: Vec<String>,
	nbar: usize,
	chan: (Sender<WriteMsg>, Receiver<WriteMsg>),
	handle: T,
}

impl MultiBar<Stdout>
{
	pub fn new() -> MultiBar<Stdout>
	{
		MultiBar::on(::std::io::stdout())
	}
}

impl<T: Write> MultiBar<T>
{
	pub fn on(handle: T) -> MultiBar<T>
	{
		MultiBar
		{
			nline: 0,
			nbar: 0,
			line: Vec::new(),
			chan: mpsc::channel(),
			handle: handle,
		}
	}

	pub fn println(&mut self, s: &str)
	{
		self.line.push(s.to_owned());
		self.nline += 1;
	}

	pub fn create_bar(&mut self, total: u64) -> ProgressBar<Pipe>
	{
		self.println("");
		self.nbar += 1;
		let mut p = ProgressBar::on(Pipe
		{
			level: self.nline - 1,
			chan: self.chan.0.clone(),
		},
		total);
	
		p.is_multi = true;
		p.add(0);
		p
	}

	pub fn listen(&mut self)
	{
		let mut first = true;
		let mut nbar = self.nbar;
		while nbar > 0
		{
			let msg = self.chan.1.recv().unwrap();
			if msg.done
			{
				nbar -= 1;
				continue;
			}
			self.line[msg.level] = msg.string;

			let mut out = String::new();
			if !first
			{
				out += &mv_csr_up(self.nline);
			}
			else
			{
				first = false;
			}
			for l in self.line.iter()
			{
				out.push_str(&format!("\r{}\n", l));
			}
			printfl!(self.handle, "{}", out);
		}
	}
}

pub struct Pipe
{
	level: usize,
	chan: Sender<WriteMsg>,
}

impl Write for Pipe
{
	fn write(&mut self, buf: &[u8]) -> Result<usize>
	{
		let s = from_utf8(buf).unwrap().to_owned();
		self.chan
			.send(WriteMsg
			{
				done: s == "",
				level: self.level,
				string: s,
			})
			.unwrap();
		Ok(1)
	}

	fn flush(&mut self) -> Result<()>
	{
		Ok(())
	}
}

struct WriteMsg
{
	done: bool,
	level: usize,
	string: String,
}
