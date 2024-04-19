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

#[derive(Debug)]
pub struct TreeItem {
    pub hash: String,
    pub file: String,
}

impl TreeItem {
    pub fn new(hash: String, file: String) -> TreeItem {
        TreeItem { hash, file }
    }

	pub fn format(&self) -> String {
		format!("{} {}", self.hash, self.file)
	}
}
