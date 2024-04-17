

use regex::Regex;
use std::fmt;

pub struct TreeParser {
	pub tree: String,
}

impl TreeParser {
	pub fn new(tree: String) -> TreeParser {
		TreeParser {
			tree,
		}
	}

	pub fn parse(&self) -> Result<Vec<TreeItem>, ParseError> {
		let mut items: Vec<TreeItem> = Vec::new();
		let mut tree_lines: Vec<&str> = self.tree.split('\n').collect();
		for line in tree_lines.iter_mut() {
			if line.is_empty() { continue; }

			// line example::
			// 100644 blob ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba    .gitignore
			let re = Regex::new(r"^\d+\s+blob\s+([a-f0-9]{40})\s+(.+)$").unwrap();
			let match_result = re.captures(line);
			if let Some(caps) = match_result {
				let hash = caps.get(1).map_or("", |m| m.as_str()).to_string();
				let file = caps.get(2).map_or("", |m| m.as_str()).to_string();
				items.push(TreeItem::new(hash, file));
			} else {
				let msg = format!("Failed to parse tree. source: '{}'", line);
				return Err(ParseError::new(msg));
			}
		}

		Ok(items)
	}
}

#[derive(Debug)]
pub struct TreeItem {
	pub hash: String,
	pub file: String,
}

impl TreeItem {
	pub fn new(hash: String, file: String) -> TreeItem {
		TreeItem {
			hash,
			file,
		}
	}
}

#[derive(Debug)]
pub struct ParseError {
	message: String
}

impl ParseError {
    pub fn new(message: String) -> ParseError {
        ParseError {
            message,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}
