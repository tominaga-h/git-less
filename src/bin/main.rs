use core::panic;

use git_less::git::{Git, tree::{GitTree, GitTreeOption}, object::RepositoryObject, parse::TreeParser};

fn main() {

	let exists = Git::exists();
	match exists {
		Ok(result) => if !result {
			panic!("git command not found.");
		}
		_ => panic!("git command not found."),
	}

	let rev = RepositoryObject::revision("HEAD".to_string());
	let option = GitTreeOption::new(true);
	let result = GitTree::exec(rev, option);
	if let Ok(output) = result {
		let parser = TreeParser::new(output);
		let items = parser.parse();
		match items {
			Ok(items) => println!("{:#?}", items),
			Err(e) => eprintln!("{}", e)
		}
	} else {
		panic!("error");
	}
}
