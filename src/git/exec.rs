use subprocess::{Exec, NullFile, ExitStatus};

pub type Result<T> = subprocess::Result<T>;

/// execute `type git` command for checking git command exists
pub fn exec_type_git() -> Result<ExitStatus> {
	Exec::cmd("type").arg("git").stdout(NullFile).join()
}

/// extract exit code from `subprocess::ExitStatus`,
/// return value when only ExitStatus is `ExitStatus::Exited`.
pub fn extract_status_code(status: ExitStatus) -> Option<u32> {
	match status {
		ExitStatus::Exited(code) => Some(code),
		_ => None,
	}
}

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
