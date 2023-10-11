pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::{done::Done, pending::Pending};

pub enum TaskTypes {
	Pending(Pending),
	Done(Done)
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> TaskTypes {
	match status {
		TaskStatus::PENDING => TaskTypes::Pending(Pending::new(title)),
		TaskStatus::DONE => TaskTypes::Done(Done::new(title)),
	}
}