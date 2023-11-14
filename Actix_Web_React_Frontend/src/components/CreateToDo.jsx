import axios from "axios";
import { useState } from "react";

function CreateToDo({passBackResponse}) {
	const [state, setState] = useState({
		title: "" 
	})
	
	const createItem = () => {
		axios
			.post(
				"http://127.0.0.1:8080/v1/item/create/" + state.title,
				{},
				{
					headers: {
						"token": localStorage.getItem("user-token"),
					}
				}
				)
				.then(res => {
					setState({ title: ""});
					passBackResponse(res)
				})
	}
	
	const handleTitleChange = (ev) => {
		setState({ title: ev.target.value })
	}
	
	return (
		<div className="inputContainer">
			<input 
				type="text" 
				id="name" 
				placeholder="Create a task"
				value={state.title}
				onChange={handleTitleChange}
			/>
			<button 
				id="create-button"
				onClick={createItem}
				type="button"	
				className="createButton"
			>
				Create
			</button>
		</div>
	)
}

export default CreateToDo;
