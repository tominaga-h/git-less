#[derive(Clone, PartialEq)]
pub enum RepositoryObjectType {
	Blob,
	Tree,
	Revision,
}

#[derive(Clone, PartialEq)]
pub struct RepositoryObject {
	pub content: String,
	pub object_type: RepositoryObjectType
}

impl RepositoryObject {
	pub fn new(content: String, object_type: RepositoryObjectType) -> RepositoryObject {
		RepositoryObject {
			content,
			object_type
		}
	}

	pub fn blob(content: String) -> RepositoryObject {
		RepositoryObject::new(content, RepositoryObjectType::Blob)
	}

	pub fn tree(content: String) -> RepositoryObject {
		RepositoryObject::new(content, RepositoryObjectType::Tree)
	}

	pub fn revision(content: String) -> RepositoryObject {
		RepositoryObject::new(content, RepositoryObjectType::Revision)
	}

	pub fn to_arg(&self) -> &str {
		self.content.as_str()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_arg() {
		let obj = RepositoryObject::blob("object".to_string());
		assert_eq!("object", obj.to_arg());
	}
}
