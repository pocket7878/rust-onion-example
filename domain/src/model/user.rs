pub type UserId = i32;

pub struct User {
		id: Option<UserId>,
		name: UserName,
}

impl User {
	pub fn new(name: UserName) -> Self {
		User {
			id: None,
			name,
		}
	}

	pub fn reconstruct(id: UserId, name: UserName) -> Self {
		User {
			id: Some(id),
			name,
		}
	}
}

pub struct UserName {
	value: String,
}

impl UserName {
	pub fn new(name: &str) -> Self {
		// TODO: Validate domain logic.
		Self {
			value: name.to_string(),
		}
	}

	pub fn reconstruct(name: &str) -> Self {
		Self {
			value: name.to_string(),
		}
	}
}