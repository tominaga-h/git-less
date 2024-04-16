#[derive(Clone, PartialEq)]
pub enum RepositoryObjectType {
	Blob,
	Tree,
	Rev,
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
		RepositoryObject::new(content, RepositoryObjectType::Rev)
	}
}
