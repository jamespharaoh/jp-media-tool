use clap::Parser as _;

mod add_subs;
mod convert;
mod dump;
mod edit;
mod info;
mod remaster;

#[ derive (clap::Parser) ]
struct MainArgs {
	#[ command (subcommand) ]
	command: Command,
}

#[ derive (clap::Subcommand) ]
enum Command {
	AddSubs (add_subs::Args),
	Convert (convert::Args),
	Dump (dump::Args),
	Edit (edit::Args),
	Info (info::Args),
	Remaster (remaster::Args),
}

pub fn main () -> anyhow::Result <()> {
	let main_args = MainArgs::parse ();
	match main_args.command {
		Command::AddSubs (add_subs_args) => add_subs::invoke (add_subs_args),
		Command::Convert (convert_args) => convert::invoke (convert_args),
		Command::Dump (dump_args) => dump::invoke (dump_args),
		Command::Edit (edit_args) => edit::invoke (edit_args),
		Command::Info (info_args) => info::invoke (info_args),
		Command::Remaster (remaster_args) => remaster::invoke (remaster_args),
	}
}
