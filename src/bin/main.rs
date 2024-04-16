use git_less::git::tree::GitTree;

fn main() {
	let result = GitTree::exec();
	match result {
		Ok(stdout) => println!("{}", stdout),
		_ => println!("error"),
	}
}
