use crate::git::{exec, object::RepositoryObject};

pub struct GitTree {}

impl GitTree {
	pub fn exec(rev: RepositoryObject, option: GitTreeOption) -> Result<String, exec::ExecuteError> {
		let output = exec::exec_ls_tree(rev, option)?;
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
