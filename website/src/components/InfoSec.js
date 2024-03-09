import React from 'react'
import "../stylesheets/prep.css"
import '../stylesheets/InfoSec.css'


const InfoSec = ( { content, color } ) => {

  return (
    <div className='infoSecContainer'>
      <div className='infoSecHeaderContainer'>
        <div className='infoSecHeaderIconContainer'>
            <span className='infoSecHeaderIcon'>{(content.Icon !== "") ? <content.icon className="infoSecIcon"/> : ""}</span></div>
        <div className='infoSecLine visibleLine' style={{background: `linear-gradient(transparent, ${color}, transparent)`, boxShadow: `0 0 10px #000000, 0 0 20px #000000, 0 0 32px ${color}, 0 0 52px ${color}`}}></div>
      </div>
        
        <div className='infoSecBody'>
          <p className='infoSecTitle' style={{color: color}}>{content.title}</p>
          <p className='infoSecHeading' style={{color: color}}>{content.heading}</p>
          <p className='infoSecBodyText'>{content.text}</p>
        </div>
    </div>
  )
}

export default InfoSec