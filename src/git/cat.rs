use crate::git::exec;

pub struct GitCat {}

impl GitCat {
	pub fn exec(hash: String) -> exec::Result<String> {
		let mut stream = exec::exec_cat_file(hash)?;
		let output = exec::get_output_from_stream(&mut stream).unwrap();
		Ok(output)
	}
}
