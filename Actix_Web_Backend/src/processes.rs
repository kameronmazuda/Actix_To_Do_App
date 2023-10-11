use super::to_do::TaskTypes;
use super::to_do::traits::{get::Get, edit::Edit, delete::Delete, create::Create};
use super::to_do::structs::{done::Done, pending::Pending};
use serde_json::{Map, Value};

fn process_pending(task: Pending, command: &str, state: &Map<String, Value>) {
	let mut state = state.clone();
    match command {
    "get" => task.get(&task.super_struct.title, &state),
    "create" => task.create(&task.super_struct.title,
    &task.super_struct.status.stringify(), &mut state),
    "edit" => task.set_to_done(&task.super_struct.title, &mut state),
    _ => println!("command: {} not supported", command)
    }
}
fn process_done(task: Done, command: &str, state: &Map<String, Value>) {
	let mut state = state.clone();
    match command {
    "get" => task.get(&task.super_struct.title, &state),
    "delete" => task.delete(&task.super_struct.title, &mut state),
    "edit" => task.set_to_pending(&task.super_struct.title, &mut state),
    _ => println!("command: {} not supported", command)
    }
}

pub fn process_input(item: TaskTypes, command: &str,
                     state: &Map<String, Value>) {
    match item {
        TaskTypes::Pending(item) => process_pending(item,
                                    command, state),
		TaskTypes::Done(item) => process_done(item,
                                    command, state)
    }
}