import axios from "axios";
import {useState} from "react";

function ToDoItem({title, status, passBackResponse, logout}) {

	const button = status === "PENDING" ? "edit" : "delete" 
	const sendRequest = () => {
		axios
			.post("http://127.0.0.1:8080/v1/item/" + button, 
			{
				"title": title,
				"status": status === "PENDING" ? "DONE" : "PENDING",
			}, 
			{
				headers: 
				{
					"token": localStorage.getItem("user-token"),
				}
			})
			.then(res => passBackResponse(res))
			.catch(err => {
				if (err.response.status === 401) {
					logout()
				}
			});
	}
	
	return (
		<div className="itemContainer">
			<p>{title}</p>
			<button 
				className="actionButton"
				onClick={sendRequest}
			>
				{button}
			</button>
		</div>
	)
}

export default ToDoItem;
