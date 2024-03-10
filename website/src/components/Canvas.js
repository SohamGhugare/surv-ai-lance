import { React, useEffect, useRef } from 'react';

const Canvas = () => {
    
    const refCanvas = useRef(null)
    useEffect(() => {
        const canvas = refCanvas.current
        const ctx = canvas.getContext("2d")
        ctx.shadowBlur = 10;
        ctx.shadowColor = "black";
        const gravity = 1
        ctx.strokeStyle = "#ffffff"
        ctx.fillStyle = "#ffffff"

        class MirroredPattern {
            constructor() {
                this.currPos = {x: 0, y: 0}
                this.prevPos = {x: 0, y: 0}
            }

        render(ctx) {
                ctx.beginPath()
            ctx.moveTo(this.currPos.x, this.currPos.y)
            ctx.lineTo()
            }
        }
    })
    return (
        <canvas ref={refCanvas} id="canvas" width="800" height="800" style={{border: "2px solid white"}}></canvas>
    )
}

export default Canvas;