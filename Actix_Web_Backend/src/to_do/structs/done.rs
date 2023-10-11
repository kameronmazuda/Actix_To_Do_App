use super::base::Base;
use super::super::enums::TaskStatus;
use super::super::traits::{get::Get, delete::Delete, edit::Edit};

pub struct Done {
	pub super_struct: Base,
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}

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