import { React, useEffect, useRef } from 'react';

const Canvas = () => {
    
    const refCanvas = useRef(null)
    useEffect(() => {
        const canvas = refCanvas.current
        const ctx = canvas.getContext("2d")
        ctx.fillRect(40, 40, 80, 80)
    })
    return (
        <canvas ref={refCanvas} id="canvas" width="800" height="800" style={{border: "2px solid white"}}></canvas>
    )
}

export default Canvas;