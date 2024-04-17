use git_less::git::{tree::{GitTree, GitTreeOption}, object::RepositoryObject};

fn main() {
	let rev = RepositoryObject::revision("HEAD".to_string());
	let option = GitTreeOption::new(true);
	let result = GitTree::exec(rev, option);
	match result {
		Ok(stdout) => println!("{}", stdout),
		_ => println!("error"),
	}
}
