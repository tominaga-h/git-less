use std::fmt;
use std::error;
use subprocess::{Exec, NullFile, ExitStatus, Redirection, CaptureData, PopenError};
use crate::git::{object::RepositoryObject, tree::GitTreeOption};


/// execute `type git` command for checking `git` command exists
pub fn exec_type_git() -> subprocess::Result<ExitStatus> {
	Exec::cmd("type").arg("git").stdout(NullFile).join()
}

/// execute `type bat` command for checking `bat` command exists
pub fn exec_type_bat() -> subprocess::Result<ExitStatus> {
	Exec::cmd("type").arg("bat").stdout(NullFile).join()
}

/// execute `type less` command for checking `less` command exists
pub fn exec_type_less() -> subprocess::Result<ExitStatus> {
	Exec::cmd("type").arg("less").stdout(NullFile).join()
}

/// execute `bat` command with `input` string to stdin
pub fn exec_bat(input: String) -> Result<(), Box<dyn error::Error>> {
	let command = format!("echo '{:?}' | bat --", input);
	let mut process = Exec::shell(command).popen()?;

	let result = process.wait()?;
	let status = extract_status_code(result).unwrap();
	if status != 0 {
		return Err(Box::new(ExecuteError::new("bat command exits as fail".to_string())));
	}
	Ok(())
}

/// execute `less` command with `input` string to stdin
pub fn exec_less(input: String) -> Result<(), Box<dyn error::Error>> {
	let command = format!("echo '{:?}' | less", input);
	let mut process = Exec::shell(command).popen()?;

	let result = process.wait()?;
	let status = extract_status_code(result).unwrap();
	if status != 0 {
		return Err(Box::new(ExecuteError::new("less command exits as fail".to_string())));
	}
	Ok(())
}

/// execute `gt ls-tree` command and return stdout string or error
pub fn exec_ls_tree(rev: RepositoryObject, option: GitTreeOption) -> Result<String, ExecuteError> {
	let command = if option.recursive {
		Exec::cmd("git").arg("ls-tree").arg("-r")
	} else {
		Exec::cmd("git").arg("ls-tree")
	};
	let result = command.arg(rev.to_arg())
		.stdout(Redirection::Pipe)
		.stderr(Redirection::Pipe)
		.capture();

	// let result = Exec::cmd("git")
	// 	.arg("ls-tree")
	// 	.arg(if option.recursive { "-r" } else { "" })
	// 	.arg(rev.to_arg())
	// 	.stdout(Redirection::Pipe)
	// 	.stderr(Redirection::Pipe)
	// 	.capture();
	process_captured_result(result)
}

/// execute `git cat-file -p` command and return stdout string or error
pub fn exec_cat_file(hash: String) -> Result<String, ExecuteError> {
	let result = Exec::cmd("git")
		.arg("cat-file")
		.arg("-p")
		.arg(hash)
		.stdout(Redirection::Pipe)
		.stderr(Redirection::Pipe)
		.capture();
	process_captured_result(result)
}

/// extract exit code from `subprocess::ExitStatus`,
/// return value when only ExitStatus is `ExitStatus::Exited`.
pub fn extract_status_code(status: ExitStatus) -> Option<u32> {
	match status {
		ExitStatus::Exited(code) => Some(code),
		_ => None,
	}
}

/// return stdout string if the `result` arg is Ok and exit status is success.
/// if the `result` arg is Err or exit status is failed, return stderr string.
pub fn process_captured_result(result: Result<CaptureData, PopenError>) -> Result<String, ExecuteError> {
	match result {
		Ok(capture_data) => {
			if capture_data.exit_status.success() {
				Ok(capture_data.stdout_str())
			} else {
				Err(ExecuteError::new(capture_data.stderr_str()))
			}
		},
		Err(e) => {
			Err(ExecuteError::new(e.to_string()))
		}
	}
}

#[derive(Debug)]
pub struct ExecuteError {
	message: String,
}

impl ExecuteError {
	pub fn new(message: String) -> ExecuteError {
		ExecuteError {
			message,
		}
	}
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ExecuteError {}

#[cfg(test)]
mod tests {
	use super::*;

	mod extract_exit_code {
		use super::*;

		#[test]
		fn return_value_only_exited() {
			let expected = 0;
			let exit_status = ExitStatus::Exited(expected);
			let actual = extract_status_code(exit_status);
			assert_eq!(Some(expected), actual);
		}
	}
}
