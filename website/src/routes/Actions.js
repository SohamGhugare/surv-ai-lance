import { React, useState, useEffect } from 'react'
import { io } from 'socket.io-client'
import "../stylesheets/Actions.css"

const Actions = () => {
    let socket

    const [videoURI, setVideoURI] = useState()

    useEffect(() => {
        socket = new WebSocket("ws://172.20.10.14:8000/ws")
        socket.addEventListener("open", e => {
            socket.send(`${Math.floor(12345 + Math.random() * 87654)}`)
        })
        socket.onmessage = encData => {
            let decData = JSON.parse(encData)
            setVideoURI(decData["footage_url"])

        }
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