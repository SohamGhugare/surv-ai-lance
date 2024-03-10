import { React, useEffect } from 'react'
import { io } from 'socket.io-client'
import "../stylesheets/Actions.css"

const Actions = () => {
    let socket
    useEffect(() => {
        socket = io("ws://localhost:5031")
    })
    return (
        <div className="actionsParentContainer">
            <video src="http://localhost:5050/videos/temp.3gp" className="actionsVideo" autoPlay={true} loop={true}/>
            <div className="actionsButtonsContainer">
                <button className="actionsRespond">Respond</button>
                <button className="actionsReject">Reject</button>
            </div>
        </div>
    )
}

export default Actions;