use regex::Regex;
use std::fmt;

pub struct TreeParser {
    pub tree: String,
}

impl TreeParser {
    pub fn new(tree: String) -> TreeParser {
        TreeParser { tree }
    }

    pub fn parse(&self) -> Result<Vec<TreeItem>, ParseError> {
        let mut items: Vec<TreeItem> = Vec::new();
        let mut tree_lines: Vec<&str> = self.tree.split('\n').collect();
        for line in tree_lines.iter_mut() {
            if line.is_empty() { continue; }
            let parse_result = TreeParser::parse_line(line);
			match parse_result {
				Ok(item) => items.push(item),
				Err(e) => return Err(e)
			}
        }
        Ok(items)
    }

    pub fn parse_line(line: &mut &str) -> Result<TreeItem, ParseError> {
        // line example::
        // 100644 blob ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba    .gitignore
        let re = Regex::new(r"^\d+\s+blob\s+([a-f0-9]{40})\s+(.+)$").unwrap();
        let match_result = re.captures(line);
        if let Some(caps) = match_result {
            let hash = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let file = caps.get(2).map_or("", |m| m.as_str()).to_string();
            Ok(TreeItem::new(hash, file))
        } else {
            let msg = format!("Failed to parse tree. source: '{}'", line);
            Err(ParseError::new(msg))
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
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: String) -> ParseError {
        ParseError { message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_line_normal() {
		let mut test_case = "100644 blob ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba    .gitignore";
		let parse_result = TreeParser::parse_line(&mut test_case);
		match parse_result {
			Ok(item) => {
				let hash_expected = "ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba".to_string();
				assert_eq!(hash_expected, item.hash);
				let file_expected = ".gitignore".to_string();
				assert_eq!(file_expected, item.file);
			},
			_ => panic!("Failed to parse line.")
		}
	}

	#[test]
	fn test_parse_line_abnormal_spaces() {
		let mut test_case = "100644     blob	   ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba .gitignore";
		let parse_result = TreeParser::parse_line(&mut test_case);
		match parse_result {
			Ok(item) => {
				let hash_expected = "ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba".to_string();
				assert_eq!(hash_expected, item.hash);
				let file_expected = ".gitignore".to_string();
				assert_eq!(file_expected, item.file);
			},
			_ => panic!("Failed to parse line.")
		}
	}

	#[test]
	#[should_panic]
	fn test_parse_line_expect_error() {
		let mut test_case = "ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba";
		let parse_result = TreeParser::parse_line(&mut test_case);
		match parse_result {
			Ok(_) => (),
			Err(_) => panic!("failed")
		}
	}
}
