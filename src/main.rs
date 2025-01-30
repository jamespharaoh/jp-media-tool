mod detect;
mod ebml;
mod ffmpeg;
mod imports;
mod matroska;
mod tool;

fn main () -> anyhow::Result <()> {
	tool::main ()
}
