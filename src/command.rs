use std::io::IsTerminal;
use clap:: {crate_name, crate_version, ColorChoice, Command, Arg, ArgAction};

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

You can input a repository object (blob, tree, branch, revision) as
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
			Arg::new("revision")
				.long("rev")
				.short('r')
				.help("A revision object such as `HEAD`, `origin/master`, etc")
		)
		.arg(
			Arg::new("recursive")
				.long("recursive")
				.short('R')
				.action(ArgAction::SetTrue)
				.help("Add `--recursive` option to `ls-tree` command")
		)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn blob_arg_exists() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "--blob", "object"]);
		assert!(matches.get_one::<String>("blob").is_some());
	}

	#[test]
	fn blob_arg_exists_shorthand() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "-b", "object"]);
		assert!(matches.get_one::<String>("blob").is_some());
	}

	#[test]
	fn blob_arg_match() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "--blob", "object"]);
		match matches.get_one::<String>("blob") {
			Some(result) => assert_eq!(result, "object"),
			None => panic!("blob arg not found"),
		}
	}

	#[test]
	fn tree_arg_exists() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "--tree", "object"]);
		assert!(matches.get_one::<String>("tree").is_some());
	}

	#[test]
	fn tree_arg_exists_shorthand() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "-t", "object"]);
		assert!(matches.get_one::<String>("tree").is_some());
	}

	#[test]
	fn tree_arg_match() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "--tree", "object"]);
		match matches.get_one::<String>("tree") {
			Some(result) => assert_eq!(result, "object"),
			None => panic!("tree arg not found"),
		}
	}

	#[test]
	fn rev_arg_exists() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "--rev", "object"]);
		assert!(matches.get_one::<String>("revision").is_some());
	}

	#[test]
	fn rev_arg_exists_shorthand() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "-r", "object"]);
		assert!(matches.get_one::<String>("revision").is_some());
	}

	#[test]
	fn rev_arg_match() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "--rev", "object"]);
		match matches.get_one::<String>("revision") {
			Some(result) => assert_eq!(result, "object"),
			None => panic!("ref arg not found"),
		}
	}

	#[test]
	fn recursive_arg_exists() {
		let command = build_command();
		let matches = command.get_matches_from(vec!["git-less", "--recursive"]);
		assert!(matches.args_present());
	}
}
