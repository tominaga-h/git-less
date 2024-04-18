pub mod cat;
pub mod exec;
pub mod tree;
pub mod parse;
pub mod object;

use subprocess;

pub struct Git {}

impl Git {
	pub fn exists() -> subprocess::Result<bool> {
		let exit_status = exec::exec_type_git()?;
		let status_code = exec::extract_status_code(exit_status).unwrap();
		if status_code == 0 {
			Ok(true)
		} else {
			Ok(false)
		}
	}
}
