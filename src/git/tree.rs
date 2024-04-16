use crate::git::{exec, object::RepositoryObject};

pub struct GitTree {}

impl GitTree {
	pub fn exec() -> exec::Result<String> {
		// TODO: replace test `HEAD`
		let rev = RepositoryObject::revision("HEAD".to_string());
		let mut stream = exec::exec_ls_tree(rev)?;
		let output = exec::get_output_from_stream(&mut stream).unwrap();
		Ok(output)
	}
}
