import React from 'react'
import "../stylesheets/prep.css"
import Login from "../components/Login"
import Banner from '../components/Banner'
import InfoSec from '../components/InfoSec'
import Navbar from '../components/Navbar'
<<<<<<< HEAD
=======
import { useScroll } from '@react-three/drei'
import SensorsIcon from '@mui/icons-material/Sensors';
import DevicesIcon from '@mui/icons-material/Devices';
import AutoAwesomeMosaicIcon from '@mui/icons-material/AutoAwesomeMosaic';
>>>>>>> 99d81a5f497914931f3b0996456b078a39287535
const Homepags = () => {

    const data = [
                {
            title: "AI / ML",
            heading: "This is some heading",
            text: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. ",
            img: "",
            reversed: false,
            icon: SensorsIcon
        }, 
        {
            title: "AI / ML",
            heading: "This is some heading",
            text: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. ",
            img: "",
            reversed: true,
            icon: SensorsIcon
        }, 
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
    <Login />
  </div>
)
}

/*  return (
    <div>
        <Navbar />
        <Banner />
        <InfoSec content={data[0]} color={"#1BB3A7"}/>
        <InfoSec content={data[1]}color={"#ECAA29"}/>
    </div>
  )
}*/

export default Homepags