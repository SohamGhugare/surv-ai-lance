import { React, useEffect } from 'react'
import "../stylesheets/InfoSec.css"

const InfoSec = ( props ) => {

  useEffect(() => {
    if (props.content.reversed) document.querySelector(".infoSecContainer").style.flexDirection = "row-reverse"
  }, [])
  return (
    <div className='infoSecContainer'>
        <img className='infoSecImage' src={props.content.img} />
        <div className='infoSecTextContainer'>
          <p className='infoSecHeading'>{props.content.title} <span className='gradientText'>{props.content.headingGradient}</span></p>
          <p className='infoSecText'>{props.content.text}</p>
        </div>
    </div>
  )
}

export default InfoSec