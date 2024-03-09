import { React, useEffect, useLayoutEffect, Suspense, useState } from 'react'
import { Canvas, useFrame } from "@react-three/fiber"
import { ScrollControls, Sky, useScroll, useGLTF, Stage } from "@react-three/drei"
import img from "../assets/images/a.png"
import "../stylesheets/Banner.css"


const Banner = () => {
    const [mousePos, setMousePos] = useState({x: 0, y: 0});
    useEffect(() => {
        window.addEventListener("mousemove", e => setMousePos({x: e.clientX, y: e.clientY}))
    });
    const Model = ( { ...props } ) => {
        const scroll = useScroll()
        const { scene, nodes } = useGLTF('/model.glb')
        useLayoutEffect(() => Object.values(nodes).forEach(node => node.recieveShadow = node.castShadow = true))
        useFrame(( state, delta ) => {              
            //-----------------------------------------------------------------------------------------------------
            const offset = (1 - scroll.offset) * 6

            
            //----------------------------------------------------------------------------------------------
            state.camera.position.set(Math.sin(offset) * -10, Math.atan(offset * Math.PI * 2) * 4, Math.cos((offset * Math.PI) / 3) * -10)
            state.camera.lookAt(0, -10, 0)
        })

        return <primitive object={scene} { ...props } />
    }

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
        <Canvas shadows camera={{position: [0, 10, 0]}} >
        <ambientLight intensity={3} />
      <spotLight angle={0.20} color="#ffd0d0" penumbra={1} position={[25, 50, -20]} shadow-mapSize={[2048, 2048]} shadow-bias={-0.0001} castShadow />
      <Suspense fallback={null}>
        <ScrollControls pages={4}>
            <Stage environment={"apartment"}>
          <Model scale={0.15}/>
          </Stage>
        </ScrollControls>
      </Suspense>
        </Canvas>
        </div>
    
    </div>
  )
}

export default Banner