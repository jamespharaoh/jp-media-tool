use clap::Parser as _;

mod dump;
mod remaster;

#[ derive (clap::Parser) ]
struct MainArgs {
	#[ command (subcommand) ]
	command: Command,
}

#[ derive (clap::Subcommand) ]
enum Command {
	Dump (dump::Args),
	Remaster (remaster::Args),
}

pub fn main () -> anyhow::Result <()> {
	let main_args = MainArgs::parse ();
	match main_args.command {
		Command::Dump (dump_args) => dump::invoke (dump_args),
		Command::Remaster (remaster_args) => remaster::invoke (remaster_args),
	}
}
