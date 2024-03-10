import { React, useEffect, useLayoutEffect, Suspense, useState } from 'react'
import { Canvas } from "@react-three/fiber"
import { PresentationControls , useGLTF , Stage } from "@react-three/drei"

import { easing } from 'maath'

const ModelRen = (props) => {
    const {scene} = new useGLTF("./earth.glb")
    return <primitive object={scene} {...props}/>
}

export default function App() {
    return (
        <Canvas dpr={[1,2]} camera={{fov:45}} style={{height:"100%",width:"100%"}}>
            <PresentationControls speed={1} zoom={1} polar={[-2 , Math.PI / 2]}>
                <Stage scale={1} environment={"apartment"}>
                    <ModelRen scale={0.2}/>
                </Stage>
            </PresentationControls>
        </Canvas>
        )
}
