import React from 'react'
import axios from 'axios'
import "../styles/LoginForm.css"

export default function LoginForm({ handleLogin }) {
    const [username, setUsername] = React.useState("")
    const [password, setPassword] = React.useState("")

    const submitLogin = (e) => {
        e.preventDefault()
        axios.post("http://127.0.0.1:8080/v1/auth/login", {
            username: username,
            password: password
        },
            {
                headers: { "Access-Control-Allow-Origin": "*" }
            }).then(res => {
                setPassword("")
                setUsername("")
                const token = JSON.parse(res.data).token;
                handleLogin(token)
            }).catch(err => {
                alert(err)
                setPassword("")
                setUsername("")
            })
    }

    const handlePasswordChange = (e) => {
        setPassword(e.target.value)
    }
    const handleUsernameChange = (e) => {
        setUsername(e.target.value)
    }

    return (
        <form className='login' onSubmit={submitLogin}>
            <h1 className='login=-title'>Login</h1>

            <input
                type="text"
                className='login-input'
                placeholder='Username'
                autoFocus
                onChange={handleUsernameChange}
                value={username}
            />

            <input 
                type="password" 
                className='login-input' 
                placeholder='Password' 
                onChange={handlePasswordChange} 
                value={password}
            />

            <input 
                type="submit" 
                className='login-button' 
                value='Login' 
            />

        </form>
    )
}
