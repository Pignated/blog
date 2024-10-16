import { useState } from 'react';

function Signin() {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    function sendSignin() {
        fetch('/api/auth/signup', 
            {
                method: "POST",
                body: JSON.stringify({username:username, password:password}),
                headers: {
                    "Content-Type": "application/json",
                },
            }
        ).then(res => console.log(res))
  
    }
    return (
        <form>
            <label> Enter your username:
                <input type="text" 
                value={username}
                onChange={(e) => setUsername(e.target.value)}/>
            </label>
            <br/>
            <label> Enter your password:
                <input type="text"
                value={password}
                onChange={(e) => setPassword(e.target.value)}/>
            </label>
            <input type="button"
            onClick={(e) => sendSignin()}
            />
        </form>
    )
}

export default Signin;