use crate::git::{exec, object::RepositoryObject};

pub struct GitTree {}

impl GitTree {
	pub fn exec(rev: RepositoryObject) -> exec::Result<String> {
		let mut stream = exec::exec_ls_tree(rev)?;
		let output = exec::get_output_from_stream(&mut stream).unwrap();
		Ok(output)
	}
}
