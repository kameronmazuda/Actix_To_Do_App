use serde::ser::{Serialize, Serializer};

#[derive(Clone)]
pub enum TaskStatus {
	DONE,
	PENDING
}

impl TaskStatus {
	pub fn stringify(&self) -> String {
		match self {
			Self::DONE => format!("DONE"),
			Self::PENDING => format!("PENDING"),
		}
	}
	 pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported",
                        input_string)
        }
    }
}

impl Serialize for TaskStatus {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer
	{
		serializer.serialize_str(&self.stringify().as_str())
	}
}
