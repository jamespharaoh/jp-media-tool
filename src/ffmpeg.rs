use crate::imports::*;

const SPIN_CHARS: & [char] = & [ '⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾' ];
const ATTR_BAR: & str = "\x1b[38;5;75m\x1b[48;5;235m";
const ATTR_RESET: & str = "\x1b[0m";

pub fn convert_progress (
	name: & str,
	duration_micros: Option <u64>,
	args: Vec <OsString>,
) -> anyhow::Result <()> {
	enum Status {
		Output (io::Result <process::Output>),
		Progress (Progress),
	}
	let (status_tx, status_rx) = mpsc::sync_channel (0);
	let progress_listener =
		ProgressListener::new ({
			let status_tx = status_tx.clone ();
			Box::new (move |progress| {
				let _ = status_tx.send (Status::Progress (progress));
			})
		}) ?;
	let progress_port = progress_listener.port ();
	thread::spawn ({
		let status_tx = status_tx.clone ();
		move || {
			let output =
				process::Command::new ("ffmpeg")
					.arg ("-hide_banner")
					.arg ("-progress")
					.arg (format! ("tcp://127.0.0.1:{progress_port}"))
					.args (args)
					.output ();
			status_tx.send (Status::Output (output)).unwrap ();
		}
	});
	let mut lines = StatusLines::new ();
	lines.push (format! ("{name}"));
	lines.flush ();
	let mut spin_idx = 0;
	let output = loop {
		match status_rx.recv () ? {
			Status::Output (output) => break output ?,
			Status::Progress (progress) => {
				for line in progress_display (name, duration_micros, & progress, & mut spin_idx) {
					lines.push (line);
				}
				lines.flush ();
			},
		}
	};
	lines.flush ();
	eprintln! ("{name}");
	progress_listener.shutdown ();
	if ! output.status.success () {
		io::stderr ().write_all (& output.stderr) ?;
		io::stderr ().write_all (& output.stdout) ?;
		if let Some (code) = output.status.code () {
			any_bail! ("Encoder process returned status {:?}", code);
		} else {
			any_bail! ("Encoder process terminated abnormally");
		}
	}
    Ok (())
}

struct StatusLines {
	num: usize,
	buf: String,
}

impl StatusLines {
	fn new () -> Self {
		Self { num: 0, buf: String::new () }
	}
	fn push (& mut self, line: impl AsRef <str>) {
		self.buf.push_str (line.as_ref ());
		self.buf.push_str ("\n");
		self.num += 1;
	}
	fn flush (& mut self) {
		{
			let mut stderr = io::stderr ();
			let _ = stderr.write_all (self.buf.as_bytes ());
			let _ = stderr.flush ();
		}
		self.buf.clear ();
		use std::fmt::Write as _;
		let _ = write! (& mut self.buf, "\r\x1b[{}A\x1b[J", self.num);
		self.num = 0;
	}
}

fn progress_display (
	name: & str,
	duration_micros: Option <u64>,
	progress: & Progress,
	spin_idx: & mut usize,
) -> Vec <String> {
	let mut lines = Vec::new ();
	let time = format! (
		"{hour:02}:{min:02}:{sec:02}",
		hour = progress.out_time_micros / 1000 / 1000 / 60 / 60,
		min = progress.out_time_micros / 1000 / 1000 / 60 % 60,
		sec = progress.out_time_micros / 1000 / 1000 % 60);
	let spin = SPIN_CHARS [* spin_idx];
	* spin_idx = (* spin_idx + 1) % SPIN_CHARS.len ();
	lines.push (format! ("{name} {spin}"));
	if let Some (duration_micros) = duration_micros {
		let progress = progress.out_time_micros as f64 / duration_micros as f64;
		lines.push (format! (
			"{ATTR_BAR}{bar}{ATTR_RESET} {val:0.2}%",
			bar = progress_bar (70, progress),
			val = progress * 100.0));
	}
	lines.push (format! (
		"frame={frame} fps={fps} bitrate={bitrate} time={time} speed={speed}",
		frame = progress.frame,
		fps = progress.fps,
		bitrate = progress.bitrate,
		speed = progress.speed));
	lines
}

fn progress_bar (mut width: usize, mut value: f64) -> String {
	if value < 0.0 { value = 0.0; }
	if 1.0 < value { value = 1.0; }
	let mut units = (value * width as f64 * 8.0) as usize;
	let mut result = String::new ();
	while 0 < width {
		match units {
			0 => { result.push (' '); },
			1 => { result.push ('▏'); units -= 1; },
			2 => { result.push ('▎'); units -= 2; },
			3 => { result.push ('▍'); units -= 3; },
			4 => { result.push ('▌'); units -= 4; },
			5 => { result.push ('▋'); units -= 5; },
			6 => { result.push ('▊'); units -= 6; },
			7 => { result.push ('▉'); units -= 7; },
			_ => { result.push ('█'); units -= 8; },
		}
		width -= 1;
	}
	result
}

#[ derive (Debug, Default) ]
struct Progress {
	frame: u64,
	fps: String,
	bitrate: String,
	total_size: String,
	out_time_micros: u64,
	out_time_str: String,
	dup_frames: u64,
	drop_frames: u64,
	speed: String,
}

struct ProgressListener {
	shutdown_tx: tok_oneshot::Sender <()>,
	port: u16,
}

impl ProgressListener {

	fn new (send_fn: Box <dyn Fn (Progress) + Send>) -> anyhow::Result <Self> {
		let listener = TcpListener::bind ("127.0.0.1:0") ?;
		let port = listener.local_addr ().unwrap ().port ();
		let (shutdown_tx, shutdown_rx) = tok_oneshot::channel ();
		thread::spawn ({
			move || {
				let runtime =
					tok_rt::Builder::new_current_thread ()
						.enable_io ()
						.build ()
						.unwrap ();
				let _guard = runtime.enter ();
				let listener = tok_net::TcpListener::from_std (listener).unwrap ();
				let task = ProgressListenerTask { listener, shutdown_rx, send_fn };
				runtime.block_on (async { task.run ().await.unwrap () });
			}
		});
		Ok (Self { shutdown_tx, port })
	}

	fn port (& self) -> u16 {
		self.port
	}

	fn shutdown (self) {
		let _ = self.shutdown_tx.send (());
	}

}

struct ProgressListenerTask {
	listener: tok_net::TcpListener,
	shutdown_rx: tok_oneshot::Receiver <()>,
	send_fn: Box <dyn Fn (Progress)>,
}

impl ProgressListenerTask {

	async fn run (mut self) -> anyhow::Result <()> {
		let socket = tokio::select! {
			_ = & mut self.shutdown_rx => return Ok (()),
			socket = self.listener.accept () => {
				let (socket, _) = socket ?;
				socket
			},
		};
		let mut reader = tok_io::BufReader::new (socket).lines ();
		let mut progress = Progress::default ();
		loop {
			tokio::select! {
				_ = & mut self.shutdown_rx => break,
				line = reader.next_line () => {
					let Ok (Some (line)) = line else { break };
					let Some (eq_pos) = line.find ('=') else { break };
					let key = & line [ .. eq_pos];
					let val = & line [eq_pos + 1 .. ];
					match key {
						"frame" => progress.frame = val.parse ().unwrap_or_default (),
						"fps" => progress.fps = val.to_owned (),
						"bitrate" => progress.bitrate = val.to_owned (),
						"total_size" => progress.total_size = val.parse ().unwrap_or_default (),
						"out_time_us" => progress.out_time_micros = val.parse ().unwrap_or_default (),
						"out_time" => progress.out_time_str = val.to_owned (),
						"dup_frames" => progress.dup_frames = val.parse ().unwrap_or_default (),
						"drop_frames" => progress.drop_frames = val.parse ().unwrap_or_default (),
						"speed" => progress.speed = val.to_owned (),
						"progress" => (self.send_fn) (mem::take (& mut progress)),
						_ => (),
					}
				},
			}
		}
		Ok (())
	}

}

pub fn probe (file_path: & Path) -> anyhow::Result <Info> {
	let mut command: Vec <OsString> = Vec::new ();
	command.push ("-hide_banner".into ());
	command.push ("-print_format".into ());
	command.push ("json".into ());
	command.push ("-show_format".into ());
	command.push ("-show_streams".into ());
	command.push ({
		let mut val = OsString::from ("file:");
		val.push (file_path);
		val
	});
	let output =
		process::Command::new ("ffprobe")
			.args (command)
			.output () ?;
	if ! output.status.success () {
		io::stderr ().write_all (& output.stderr) ?;
		any_bail! ("Error invoking ffprobe: {}", output.status);
	}
	let data: FfData = serde_json::from_slice (& output.stdout) ?;
	Ok (Info {
		duration: data.format.duration,
		streams: data.streams.iter ()
			.map (|stream| Ok (Stream {
				stream_type: match & * stream.codec_type {
					"audio" => StreamType::Audio,
					"data" => StreamType::Data,
					"subtitle" => StreamType::Subtitle,
					"video" => StreamType::Video,
					codec_type => any_bail! ("Invalid codec type: {codec_type}"),
				},
				codec_name: stream.codec_name.clone (),
			}))
			.try_collect () ?,
	})
}

#[ derive (Debug) ]
pub struct Info {
	pub duration: f64,
	pub streams: Vec <Stream>,
}

#[ derive (Debug) ]
pub struct Stream {
	pub stream_type: StreamType,
	pub codec_name: String,
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum StreamType {
	Audio,
	Data,
	Subtitle,
	Video,
}

#[ derive (Debug, Deserialize) ]
struct FfData {
	format: FfFormat,
	streams: Vec <FfStream>,
}

#[ serde_as ]
#[ derive (Debug, Deserialize) ]
struct FfFormat {
	#[ serde_as (as = "DisplayFromStr") ]
	duration: f64,
}

#[ derive (Debug, Deserialize) ]
struct FfStream {
	codec_name: String,
	codec_type: String,
}
