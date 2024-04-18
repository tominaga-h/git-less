use crate::git::exec;

pub struct GitCat {}

impl GitCat {
	pub fn exec(hash: String) -> Result<String, exec::ExecuteError> {
		let output = exec::exec_cat_file(hash)?;
		Ok(output)
	}
}
