use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Done {
	pub super_struct: Base,
}

impl Done {
	pub fn new(title: &str) -> Self {
		Self { 
			super_struct: Base { 
				title: title.to_string(), 
				status: TaskStatus::DONE
			}
		}
	}
}
