import { React, useEffect } from 'react'
import "../stylesheets/prep.css"
import Banner from '../components/Banner'
import InfoSec from '../components/InfoSec'
import Navbar from '../components/Navbar'
import { useScroll } from '@react-three/drei'
import SensorsIcon from '@mui/icons-material/Sensors';
import DevicesIcon from '@mui/icons-material/Devices';
import AutoAwesomeMosaicIcon from '@mui/icons-material/AutoAwesomeMosaic';
import ThreeSection from '../components/ThreeSection'
import Canvas from "../components/Canvas"

const Homepage = () => {

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
            title: "CDN",
            heading: "This is some CDN HEADING",
            text: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. ",
            img: "",
            reversed: true,
            icon: SensorsIcon
        }, 
    ]
    

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            entry.target.querySelector(".infoSecLine").classList.toggle("visibleLine", entry.isIntersecting)
            entry.target.querySelector(".infoSecBodyText").classList.toggle("fadeUpAnimation", entry.isIntersecting)
            entry.target.querySelector(".infoSecHeading").classList.toggle("fadeInAnimation", entry.isIntersecting)
            entry.target.querySelector(".infoSecHeading").classList.toggle("expandText", entry.isIntersecting)
            entry.target.querySelector(".infoSecIcon").classList.toggle("fadeInAnimation", entry.isIntersecting)
            entry.target.querySelector(".infoSecTitle").classList.toggle("fadeInAnimation", entry.isIntersecting)
            if (entry.isIntersecting) observer.unobserve(entry.target)
        })
    }, { threshold: 0.5 });

    const threeObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            entry.target.querySelector(".threeSectionWord").classList.toggle("animatedUnderline", entry.isIntersecting)
            if (entry.isIntersecting) threeObserver.unobserve(entry.target)
        })
    }, { threshold: 0.5 })

    useEffect(() => {

        let targets = document.getElementsByClassName("infoSecContainer")
        for (const target of targets) {
            observer.observe(target)
        }

        targets = document.getElementsByClassName("threeSectionContainer")
        for (const target of targets) {
            threeObserver.observe(target)
        }


    }, [])


  return (

    <div>
        <Banner />
        <InfoSec content={{icon: "none"}} color={"#1ff3A7"}/>
        <InfoSec content={data[0]} color={"#1BB3A7"}/>
        <InfoSec content={data[1]}color={"#ECAA29"}/>
        <InfoSec content={data[1]}color={"#C91C50"}/>
        <ThreeSection />
        <Canvas />

    </div>
  )
}

export default Homepage