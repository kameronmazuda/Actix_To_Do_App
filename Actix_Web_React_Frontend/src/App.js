import {useEffect, useState} from "react";
import axios from "axios";

function App() {
	let [message, setMessage] = useState("To Do App");
	let [pendingItems, setPendingItems] = useState([]);
	let [doneItems, setDoneItems] = useState([]);
	let [pendingItemsCount, setPendingItemsCount] = useState(0);
	let [doneItemsCount, setDoneItemsCount] = useState(0);
	
	useEffect(() => {
		axios.get("http://127.0.0.1:8080/v1/item/get", {
			headers: {
				"token": "some_token"
			}
		}).then(
			res => {
				setPendingItems(res.data["pending_items"])
				setDoneItems(res.data["done_items"])
			}
		)
	}, [])

	
  return (
	<div>
		{message}	
		<div>
			<h3>Completed Tasks</h3>
			<ul>
				{pendingItems.map(({title, status}, i) => (
					<li key={i}>{title} {status}</li>
				))}
			</ul>
		</div>
		<div>
			<h3>Done Tasks</h3>
			<ul>
				{doneItems.map(({title, status}, i) => (
					<li key={i}>{title} {status}</li>
				))}
			</ul>
		</div>
	</div>
  );
}

export default App;
