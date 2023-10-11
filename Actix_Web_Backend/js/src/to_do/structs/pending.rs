use super::base::Base;
use super::super::enums::TaskStatus;
use super::super::traits::{get::Get, create::Create, edit::Edit};

pub struct Pending {
	pub super_struct: Base,
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}

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