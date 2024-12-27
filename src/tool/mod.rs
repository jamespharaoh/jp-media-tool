use clap::Parser as _;

mod convert;
mod dump;

#[ derive (clap::Parser) ]
struct MainArgs {
	#[ command (subcommand) ]
	command: Command,
}

#[ derive (clap::Subcommand) ]
enum Command {
	Convert (convert::Args),
	Dump (dump::Args),
}

pub fn main () -> anyhow::Result <()> {
	let main_args = MainArgs::parse ();
	match main_args.command {
		Command::Convert (convert_args) => convert::invoke (convert_args),
		Command::Dump (dump_args) => dump::invoke (dump_args),
	}
}
