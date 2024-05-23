use std::io::{self, ErrorKind};
use git_less::{select, reader, command};
use git_less::git::{
	Git,
	tree::{GitTree, GitTreeOption},
	cat::GitCat,
	object::RepositoryObject,
	parse::TreeParser
};

fn app() -> Result<(), Box<dyn std::error::Error>> {

	let exists = Git::exists();
	match exists {
		Ok(result) => if !result {
			panic!("git command not found.");
		}
		_ => panic!("git command not found."),
	}

	// command line arguments
	let command = command::build_command();
	let args = command.get_matches();
	let recursive = args.get_flag("recursive");
	let revision = args.get_one::<String>("revision");

	// blob引数がある場合はls-tree選択処理をスキップ
	if let Some(blob) = args.get_one::<String>("blob") {
		let contents = GitCat::exec(blob.to_owned())?;
		let reader = reader::Reader::new();
		reader.read(contents)?;
		return Ok(());
	}

	// Revisionがなかったらエラー
	if revision.is_none() {
		return Err(Box::new(io::Error::new(ErrorKind::NotFound, "If you don't input the blob option, you have to input the revision option.")));
	}

	let rev = RepositoryObject::revision(revision.unwrap().to_owned());

	// ls-tree
	let option = GitTreeOption::new(recursive);
	let output = GitTree::exec(rev, option)?;

	// select
	let selected = select::skim_select(output);
	if selected.is_empty() {
		return Err(Box::new(io::Error::new(ErrorKind::NotFound, "No item selected.")));
	}

	// parse
	let parser = TreeParser::new(selected[0].output().to_string());
	let items = parser.parse()?;
	let contents = GitCat::exec(items[0].hash.clone())?;

	// read
	let reader = reader::Reader::new();
	reader.read(contents)?;
	Ok(())
}

fn main() {
	let result = app();
	match result {
		Ok(_) => (),
		Err(e) => eprint!("Error: {}", e)
	}
}
