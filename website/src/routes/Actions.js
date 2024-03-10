import { React, useState, useEffect, useRef } from 'react'
import { io } from 'socket.io-client'
import "../stylesheets/Actions.css"

const Actions = () => {
    let socket
    const [caseData, setCaseData] = useState({})
    let videoRef = useRef(null)
    const play = () => {
        videoRef.current.play()
        console.log("Playing")
    }

    const reject = () => {
        setCaseData({})
        videoRef.current.src = ""
    }


    useEffect(() => {
        socket = new WebSocket("ws://172.20.10.14:8000/ws")
        socket.addEventListener("open", e => {
            socket.send(`${Math.floor(12345 + Math.random() * 87654)}`)
        })
        socket.onmessage = encData => {
            console.log(encData.data)
            let decData = JSON.parse(encData["data"])
            setCaseData(decData)

        }
    })
    return (
        <div className="actionsParentContainer">
            <div className="actionsVideoContainer">
                <video ref={videoRef} src={caseData["footage_url"]} className="actionsVideo" autoPlay={true} loop={true} />
                <div className="actionsVideoTextContainer">
                    <p><span className="coloredText">Camera ID: </span>{caseData["camera_id"]}</p>
                    <p><span className="coloredText">Location: </span>  {caseData["camera_location"]}</p>
                    <p><span className="coloredText">Distance: </span> {Object.keys(caseData).length == 0 ? "" : "2km"}</p>
                    <p><span className="coloredText">Video URI: </span> {caseData["footage_url"]}</p>
                    <p><span className="coloredText">Timstamp: </span> {caseData["timestamp"]}</p>
                </div>
            </div>
            <div className="actionsButtonsContainer">
                <button className="actionsRespond" onClick={play}>Respond</button>
                <button className="actionsReject" onClick={reject}>Reject</button>
            </div>
        </div>
    )
}

export default Actions;