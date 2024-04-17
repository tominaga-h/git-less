use crate::git::{exec, object::RepositoryObject};

pub struct GitTree {}

impl GitTree {
	pub fn exec(rev: RepositoryObject, option: GitTreeOption) -> exec::Result<String> {
		let mut stream = exec::exec_ls_tree(rev, option)?;
		let output = exec::get_output_from_stream(&mut stream).unwrap();
		Ok(output)
	}
}

pub struct GitTreeOption {
	pub recursive: bool,
}

impl GitTreeOption {
	pub fn new(recursive: bool) -> GitTreeOption {
		GitTreeOption {
			recursive,
		}
	}
}
