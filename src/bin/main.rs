use git_less::git::{tree::GitTree, object::RepositoryObject};

fn main() {
	let rev = RepositoryObject::revision("HEAD".to_string());
	let result = GitTree::exec(rev);
	match result {
		Ok(stdout) => println!("{}", stdout),
		_ => println!("error"),
	}
}
