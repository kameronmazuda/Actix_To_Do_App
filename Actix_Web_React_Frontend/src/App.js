import "./App.css"

import {useEffect, useState} from "react";
import axios from "axios";

import ToDoItem from "./components/ToDoItem";
import CreateToDo from "./components/CreateToDo";
import LoginForm from "./components/LoginForm";

function App() {
	const [pendingItems, setPendingItems] = useState([]);
	const [doneItems, setDoneItems] = useState([]);
	const [pendingItemsCount, setPendingItemsCount] = useState(0);
	const [doneItemsCount, setDoneItemsCount] = useState(0);
	const [loginStatus, setLoginStatus] = useState(false);
	
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
					logout={logout}
				/>
			)
		})
		return itemList;
	}
	
	const handleReturnedState = (res) => {
		setPendingItems(processItemValues(res.data["pending_items"]));
		setDoneItems(processItemValues(res.data["done_items"]));
		setDoneItemsCount(res.data["done_items_count"])
		setPendingItemsCount(res.data["pending_items_count"])
		console.log("new state", res.data)
	}

	const logout = () => {
        localStorage.removeItem("user-token");
        setLoginStatus(false);
    }

	const handleLogin = (token) => {
		try {
			console.log("token: ", token)
			localStorage.setItem("user-token", token);
			setLoginStatus(true);
		} catch (error) {
			console.log(error)
		}

	}
	
	useEffect(() => {
		const userToken = localStorage.getItem("user-token");

		if (userToken) {
			setLoginStatus(true);
		} else {
			setLoginStatus(false);
		}

		axios.get("http://127.0.0.1:8080/v1/item/get", {
			headers: {
				"token": localStorage.getItem("user-token")
			}
		}).then(res => {
			setPendingItems(processItemValues(res.data["pending_items"]));
			setDoneItems(processItemValues(res.data["done_items"]));
			setDoneItemsCount(res.data["done_item_count"]);
			setPendingItemsCount(res.data["pending_item_count"]);
			console.log("old state", res.data)
		}
		).catch(err => {
			if (err.response.status === 401) {
				console.log(err.response.data)
                logout()
			}
		})
	}, [loginStatus])
	
  	if (!loginStatus) {
		return (
			<div className="App">
				<div className="mainContainer">
					<LoginForm handleLogin={handleLogin} />
                </div>
			</div>
		)
	} 

	return (
		<div className="App">
			<header className="header">
				<nav className="navbar">
					<div className="navbar-brand-container">
						<a className="navbar-brand" href="#">To-Do List</a>
					</div>
					<ul className="navbar-list">
						<li className="nav-list-item">
							<a className="navbar-menu-item" href="#">To-Do List</a>
						</li>
						<li className="nav-list-item">
							<a className="navbar-menu-item" href="#">To-Do List</a>
						</li>
					</ul>
				</nav>
				<div className="counter">
					<div className="tasksNumContainer">
						Complete tasks: 
						<span id="completeNum">{doneItemsCount}</span>
					</div>
					<div className="tasksNumContainer">
					Pending tasks: <span id="pendingNum">{pendingItemsCount}</span></div>
				</div>
			</header>
			<div className="mainContainer">
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
