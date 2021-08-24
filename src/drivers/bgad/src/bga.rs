use syscall::io::{Io, Pio};
const BGA_IDX_XRES: u16 = 1;
const BGA_IDX_YRES: u16 = 2;
const BGA_IDX_BPP: u16 = 3;
const BGA_IDX_ENAB: u16 = 4;

pub struct Bga
{
	idx: Pio<u16>,
	data: Pio<u16>,
}

impl Bga
{
	pub fn new() -> Bga
	{
		Bga
		{
			idx: Pio::new(0x1CE),
			data: Pio::new(0x1CF),
		}
	}

	fn read(&mut self, idx: u16) -> u16
	{
		self.idx.write(idx);
		self.data.write(data);
	}

	pub fn width(&mut self) -> u16
	{
		self.read(BGA_IDX_XRES)
	}

	pub fn height(&mut self) -> u16
	{
		self.read(BGA_IDX_YRES)
	}

	#[allow(dead_code)]
	pub fn setsize(&mut self, width: u16, height: u16)
	{
		self.write(BGA_IDX_ENAB, 0);
		self.write(BGA_IDX_XRES, width);
		self.write(BGA_IDX_YRES, height);
		self.write(BGA_IDX_BPP, 32);
		self.write(BGA_IDX_ENAB, 0x41);
	}
}
