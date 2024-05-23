use std::error::Error;
use crate::exec;

const COMMAND_BAT: &str = "bat";
const COMMAND_LESS: &str = "less";

pub struct Reader {
	pub command: String
}

impl Default for Reader {
	fn default() -> Self {
		Reader::new()
	}
}

impl Reader {
	pub fn new() -> Reader {
		let mut command: String = String::from("");

		// 1. check `bat` command exists
		let bat_exists: bool = {
			let exit_status = exec::exec_type_bat();
			match exit_status {
				Ok(status) => {
					let status_code = exec::extract_status_code(status).unwrap();
					status_code == 0
				},
				Err(_) => false
			}
		};

		if bat_exists {
			command = COMMAND_BAT.to_string();
		}
		else {
			// 2. check `less` command exists
			let less_exists: bool = {
				let exit_status = exec::exec_type_less();
				match exit_status {
					Ok(status) => {
						let status_code = exec::extract_status_code(status).unwrap();
						status_code == 0
					},
					Err(_) => false
				}
			};

			if less_exists {
				command = COMMAND_LESS.to_string();
			}
		}

		Reader { command }
	}

	pub fn read(&self, contents: String) -> Result<(), Box<dyn Error>> {
		if self.command == COMMAND_BAT {
			exec::exec_bat(contents.replace('`', r"\`"))
		} else {
			exec::exec_less(contents.replace('`', r"\`"))
		}
	}
}
