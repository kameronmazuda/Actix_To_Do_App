import "./App.css"

import {useEffect, useState} from "react";
import axios from "axios";

import ToDoItem from "./components/ToDoItem";
import CreateToDo from "./components/CreateToDo";

function App() {
	let [message, setMessage] = useState("To Do App");
	let [pendingItems, setPendingItems] = useState([]);
	let [doneItems, setDoneItems] = useState([]);
	let [pendingItemsCount, setPendingItemsCount] = useState(0);
	let [doneItemsCount, setDoneItemsCount] = useState(0);
	
	
	const processItemValues = (items) => {
		let itemList = [];
		items?.forEach((item, i) => {
			itemList.push(
				<ToDoItem 
					key={i}
					title={item.title}
					status={item.status}
					passBackResponse={
						handleReturnedState
					}
				/>
			)
		})
		return itemList;
	}
	
	const handleReturnedState = (res) => {
		let pending_items = res.data["pending_items"];
		let done_items = res.data["done_items"];
		setPendingItems(processItemValues(pending_items));
		setDoneItems(processItemValues(done_items));
		setDoneItemsCount(res.data["done_items_count"])
		setPendingItemsCount(res.data["pending_items_count"])
		console.log("new state", res.data)
	}
	
	
	useEffect(() => {
		axios.get("http://127.0.0.1:8080/v1/item/get", {
			headers: {
				"token": "some_token"
			}
		}).then(res => {
				let pending_items = res.data["pending_items"];
				let done_items = res.data["done_items"];
				setPendingItems(processItemValues(pending_items));
				setDoneItems(processItemValues(done_items));
				setDoneItemsCount(res.data["done_item_count"]);
				setPendingItemsCount(res.data["pending_item_count"]);
		console.log("old state", res.data)
			}
		)
	}, [])

	
  return (
	<div className="App">
		<div className="mainContainer">
			<div className="header">
				<div className="tasksNumContainer">
					Complete tasks: 
					<span id="completeNum">{doneItemsCount}</span>
				</div>
				<div className="tasksNumContainer">
				Pending tasks: <span id="pendingNum">{pendingItemsCount}</span></div>
			</div>
			<div className="items">
				<h3 className="items_title">Pending Tasks</h3>
				{pendingItems.map((item, i) => (
				<div key={i}>
						{item}
				</div>
				))}
			</div>
			<div className="items">
				<h3 className="items_title">Done Tasks</h3>
				{doneItems.map((item, i) => (
					<div key={i}>
						{item}
					</div>
				))}
			</div>
		<div>
			<CreateToDo passBackResponse={handleReturnedState} />
		</div>
		</div>
	</div>
  );
}

export default App;
