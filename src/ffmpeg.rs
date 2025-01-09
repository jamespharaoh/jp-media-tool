use crate::imports::*;

const SPIN_CHARS: & [char] = & [ '⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾' ];
const PREV_LINE: & str = "\x1b[A\x1b[2K\r";

pub fn convert_progress (
	name: & str,
	duration_micros: Option <u64>,
	args: Vec <OsString>,
) -> anyhow::Result <()> {
	enum Status {
		Output (io::Result <process::Output>),
		Progress (Vec <(String, String)>),
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
	eprintln! ("{name}");
	let mut lines = Vec::new ();
	let mut spin_idx = 0;
	let output = loop {
		match status_rx.recv () ? {
			Status::Output (output) => break output ?,
			Status::Progress (progress) => {
				for _ in 0 .. lines.len () { eprint! ("{PREV_LINE}"); }
				lines = progress_display (duration_micros, & progress, & mut spin_idx);
				for line in & lines { eprintln! ("{line}"); }
			},
		}
	};
	for _ in 0 .. lines.len () { eprint! ("{PREV_LINE}"); }
	progress_listener.shutdown ();
	if ! output.status.success () {
		io::stderr ().write_all (& output.stdout) ?;
		if let Some (code) = output.status.code () {
			any_bail! ("Encoder process returned status {:?}", code);
		} else {
			any_bail! ("Encoder process terminated abnormally");
		}
	}
    Ok (())
}

fn progress_display (
	duration_micros: Option <u64>,
	fields: & [(String, String)],
	spin_idx: & mut usize,
) -> Vec <String> {
	let mut lines = Vec::new ();
	let get = |name, default|
		fields.iter ()
			.find (|(k, _)| k == name)
			.map (|(_, v)| v.as_str ())
					.unwrap_or (default);
	let frame = get ("frame", "");
	let fps = get ("fps", "");
	let bitrate = get ("bitrate", "");
	let out_time_us: u64 = get ("out_time_us", "0").parse ().unwrap_or (0);
	let speed = get ("speed", "");
	let time = format! (
		"{hour:02}:{min:02}:{sec:02}",
		hour = out_time_us / 1000 / 1000 / 60 / 60,
		min = out_time_us / 1000 / 1000 / 60 % 60,
		sec = out_time_us / 1000 / 1000 % 60);
	if let Some (duration_micros) = duration_micros {
		let progress = out_time_us as f64 / duration_micros as f64;
		lines.push (format! ("[{}] {:0.2}%", progress_bar (60, progress), progress * 100.0));
	}
	let spin = SPIN_CHARS [* spin_idx];
	* spin_idx = (* spin_idx + 1) % SPIN_CHARS.len ();
	lines.push (format! ("frame={frame} fps={fps} bitrate={bitrate} time={time} speed={speed} {spin}"));
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

struct ProgressListener {
	shutdown_tx: tok_oneshot::Sender <()>,
	port: u16,
}

impl ProgressListener {

	fn new (send_fn: Box <dyn Fn (Vec <(String, String)>) + Send>) -> anyhow::Result <Self> {
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
	send_fn: Box <dyn Fn (Vec <(String, String)>)>,
}

impl ProgressListenerTask {

	async fn run (mut self) -> anyhow::Result <()> {
		let (socket, _) = self.listener.accept ().await ?;
		let mut reader = tok_io::BufReader::new (socket).lines ();
		let mut buf = Vec::new ();
		loop {
			tokio::select! {
				_ = & mut self.shutdown_rx => break,
				line = reader.next_line () => {
					let Ok (Some (line)) = line else { break };
					let Some (eq_pos) = line.find ('=') else { break };
					let key = line [ .. eq_pos].to_owned ();
					let val = line [eq_pos + 1 .. ].to_owned ();
					let last_line = & key == "progress";
					buf.push ((key, val));
					if last_line { (self.send_fn) (mem::take (& mut buf)); }
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
	command.push (file_path.into ());
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
					"video" => StreamType::Video,
					"audio" => StreamType::Audio,
					"subtitle" => StreamType::Subtitle,
					codec_type => any_bail! ("Invalid codec type: {codec_type}"),
				},
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
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum StreamType {
	Video,
	Audio,
	Subtitle,
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
	codec_type: String,
}
