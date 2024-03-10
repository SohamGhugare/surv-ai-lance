import React from 'react'
import "../stylesheets/prep.css"
import "../stylesheets/Login.css"

const Login = () => {
    return (
        <div className='loginContainer'>
            <div className='left'>
                <h1>Sign In to <br />Receive Alerts</h1>
            </div>
            <div>
            <form className='right' action="/url" method="GET">
                <div className='inputContainer'>
                    <input type="email" placeholder="Email ID" />
                </div>
                <div className='inputContainer'>
                    <input type="password" placeholder="Password" />
                </div>
                <div className='forgot-password'><p>Recover password?</p></div>
                <div className='submit'><p>Sign In</p></div>
            </form>
            </div>
        </div>
    )
}
export default Login