use core::panic;

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

	let rev = RepositoryObject::revision("HEAD".to_string());
	let option = GitTreeOption::new(true);
	let output = GitTree::exec(rev, option)?;
	let parser = TreeParser::new(output);
	let items = parser.parse()?;
	let hash: String = items[0].hash.clone();

	let contents = GitCat::exec(hash.clone())?;
	println!("{}", contents);
	println!("target hash: '{}'", hash);
	Ok(())
}

fn main() {
	let result = app();
	match result {
		Ok(_) => (),
		Err(e) => eprintln!("{}", e)
	}
}
