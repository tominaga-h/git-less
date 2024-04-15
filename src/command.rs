use std::io::IsTerminal;
use clap:: {crate_name, crate_version, ColorChoice, Command, Arg};

pub fn build_command() -> Command {
	let interactive_output = std::io::stdout().is_terminal();
	let color_when: ColorChoice = {
		if interactive_output{
			ColorChoice::Auto
		} else {
			ColorChoice::Never
		}
	};

	Command::new(crate_name!())
		.version(crate_version!())
		.color(color_when)
		.max_term_width(100)
		.about("A git-depended tool for read file contents.")
		.long_about(r#"A git-depended tool for read file contents.

You can input a repository object (blob, tree, branch, reference) as
a command line argument. If you input a blob object, you can read its contents
with the `less` command. If you input a tree, branch, or ref object, you can
read the contents of the selected file with the `less` command."#)
		.arg(
			Arg::new("blob")
				.long("blob")
				.short('b')
				.help("A blob object")
		)
		.arg(
			Arg::new("tree")
				.long("tree")
				.short('t')
				.help("A tree object")
		)
		.arg(
			Arg::new("reference")
				.long("reference")
				.short('r')
				.help("A reference object such as `HEAD`, `origin/master`, etc")
		)
		.arg(
			Arg::new("recursive")
				.long("recursive")
				.short('R')
				.help("Add `--recursive` option to `ls-tree` command")
		)
}
