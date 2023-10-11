use serde_json::{Value, Map};
use crate::state::write_to_file;
use crate::FILE_PATHNAME;

pub trait Delete {
    fn delete(&self, title: &String,
        state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file(FILE_PATHNAME, state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}