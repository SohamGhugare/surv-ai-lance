import { React, useEffect, useLayoutEffect, Suspense, useState, useRef } from 'react'
import { Canvas, useFrame } from "@react-three/fiber"
import { ScrollControls, Sky, useScroll, useGLTF, Stage } from "@react-three/drei"
import img from "../assets/images/a.png"
import "../stylesheets/Banner.css"
import ThreeModel from "./ThreeModel"

const Banner = () => {


  return (
    <div className='bannerParent' style={{"backgroundImage": `url(${img})`, backgroundSize: "cover"}}>
        <div className='bannerTextContainer'>
            <p className='bannerTitle fadeUpAnimation'>
                Surv-<span className='coloredText'>AI</span><span>-lance</span>
            </p>
            <span className='bannerSubTitle fadeUpAnimation'>
                Looking after you. Always.
            </span>
            <p className='bannerText fadeLeftAnimation'>
            An attempt at making the streets safer. Our mission is simple: ensuring swift detection and response to violence for safer communities and to revolutionize surveillance.
            </p>
        </div>
        <div className='canvas fadeLeftAnimation'>
        </div>
    
    </div>
  )
}

export default Banner