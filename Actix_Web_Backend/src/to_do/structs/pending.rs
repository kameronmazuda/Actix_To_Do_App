use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Pending {
	pub super_struct: Base,
}

impl Pending {
	pub fn new(title: &str) -> Self {
		Self { 
			super_struct: Base { 
				title: title.to_string(), 
				status: TaskStatus::PENDING
			}
		}
	}
}
