import React from 'react'
import "../stylesheets/prep.css"
import Banner from '../components/Banner'
import InfoSec from '../components/InfoSec'
import Navbar from '../components/Navbar'
import { useScroll } from '@react-three/drei'
const Homepags = () => {

    const text = [
        {
            "title": "AI / ML",
            "text": "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. ",
            "img": ""
        }
    ]

    const infoSectionObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
          entry.target.querySelector(".infoSecImage").classList.toggle("fadeUpAnimation", entry.isIntersecting)
          entry.target.querySelector(".infoSecHeading").classList.toggle("fadeLeftAnimation", entry.isIntersecting)
          entry.target.querySelector(".infoSecText").classList.toggle("fadeLeftAnimation", entry.isIntersecting)
          if ( entry.isIntersecting ) infoSectionObserver.unobserve(entry.target)
        })
      }, { threshold: 0.4 })


  return (
    <div>
        <Banner />
        <InfoSec content={text}/>
    </div>
  )
}

export default Homepags