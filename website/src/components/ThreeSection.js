import React from 'react'
import ThreeModel from './ThreeModel'
import "../stylesheets/ThreeSection.css"

const ThreeSection = () => {
  return (
    <div className="threeSectionContainer">
      <div className="threeSectionTextContainer">
        <p className="threeSectionTitle">Safer <span className="threeSectionWord gradientText">Streets</span> with AI</p>
        <p className="threeSectionText">The best possible use of AI there possibly could be is helping people stay safe. At <span className="coloredText underline">Omniscience</span>, That is our mission.</p>
      </div>
        <ThreeModel />
    </div>
  )
}

export default ThreeSection