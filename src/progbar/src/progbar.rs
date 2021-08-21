use std::io::{self, Write};
use std::iter::repeat;
use std::time::Duration;
use time::{self, SteadyTime};
use std::io::Stdout;
use tty::{Wide, termsize};

macro_rules! kbfmt
{
	($n: ident) => {{
		let kb = 1024f64;
		match $n
		{
			$n if $n >= kb.powf(4_f64) => format!("{:.*} TB", 2, $n / kb.powf(4_f64)),
			$n if $n >= kb.powf(3_f64) => format!("{:.*} GB", 2, $n / kb.powf(3_f64)),
			$n if $n >= kb.powf(2_f64) => format!("{:.*} MB", 2, $n / kb.powf(2_f64)),
			$n if $n >= kb => format!("{:.*} KB", 2, $n / kb),
			_ => format!("{:.*} B", 0, $n)
			}
		}	
	}
}

macro_rules! repeat
{
	($s: expr, $n: expr) => {{
		&repeat($s).take($n).collect::<String>()
	}}
}

const FMT: &'static str = "[=>-]";
const TICKFMT: &'static str = "\\|/-";
const NANO_PERSEC: u32 = 1_000_000_000;

#[derive(Debug)]
pub enum Units
{
	Default,
	Bytes,
}

pub struct ProgressBar<T: Write>
{
	start_tm: SteadyTime,
	units: Units,
	pub total: u64,
	curr: u64,
	bar_start: String,
	bar_curr: String,
	bar_curr_n: String,
	bar_rem: String,
	bar_end: String,
	tick: Vec<String>,
	tick_st: usize,
	width: Option<usize>,
	msg: String,
	last_ref_tm: SteadyTime,
	maxim_ref_rate: Option<time::Duration>,
	pub is_finished: bool,
	pub is_multibar: bool,
	pub vis_bar: bool,
	pub vis_spd: bool,
	pub vis_percent: bool,
	pub vis_count: bool,
	pub vis_remtime: bool,
	pub vis_tick: bool,
	pub vis_msg: bool,
	handle: T,
}

impl ProgressBar<Stdout>
{
	pub fn new(total: u64) -> ProgressBar<Stdout>
	{
		let handle = ::std::io::stdout();
		ProgressBar::on(handle, total)
	}
}

impl<T: Write> ProgressBar<T>
{
	pub fn on(handle: T, total: u64) -> ProgressBar<T>
	{
		let mut progbar = ProgressBar
		{
			total: total,
			curr: 0,
			start_tm: SteadyTime::now(),
			units: Units::Default,
			pub is_finished: false,
			pub is_multibar: false,
			pub vis_bar: true,
			pub vis_spd: true,
			pub vis_percent: true,
			pub vis_count: true,
			pub vis_remtime: true,
			pub vis_tick: false,
			pub vis_msg: true,
			bar_start: String::new(),
			bar_curr: String::new(),
			bar_curr_n: String::new(),
			bar_rem: String::new(),
			bar_end: String::new(),
			tick: Vec::new(),
			tick_st: 0,
			width: None,
			msg: String::new(),
			last_ref_tm: SteadyTime::now(),
			maxim_ref_rate: None,
			handle: handle,
		};
		progbar.fmt(FMT);
		progbar.tick_fmt(TICKFMT);
		progbar
	}

	pub fn unit_set(&mut self, u: Units)
	{
		self.units = u;
	}

	pub fn fmt(&mut self, fmt: &str)
	{
		if fmt.len() >= 5
		{
			let v: Vec<&str> = fmt.split("").collect();
			self.bar_start = v[1].to_owned();
			self.bar_curr = v[2].to_owned();
			self.bar_curr_n = v[3].to_owned();
			self.bar_rem = v[4].to_owned();
			self.bar_end = v[5].to_owned();
		}
	}		

	pub fn msg(&mut self, msg: &str)
	{
		self.msg = msg.to_owned().replace("\n", " ").replace("\r", " ")
	}

	pub fn tickfmt(&mut self, tickfmt: &str)
	{
		if tickfmt != TICKFMT
		{
			self.vis_tick = true;
		}
		self.tick = tickfmt.split("").map(|x| x.to_owned()).filter(|x| x != "").collect();
	}

	pub fn width_set(&mut self, w: Option<usize>)
	{
		self.width = w;
	}

	pub fn maxim_refrate_set(&mut self, w: Option<Duration>)
	{
		self.maxim_refrate = w.map(time::Duration::from_std).map(Result::
