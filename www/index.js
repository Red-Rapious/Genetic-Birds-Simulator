import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();

const viewport = document.getElementById("viewport");

const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';


const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size, rotation) {
    this.beginPath();

    this.moveTo(
        x + Math.sin(-rotation) * size * 1.5,
        y + Math.cos(-rotation) * size * 1.5,
    );
    this.lineTo(
        x + Math.sin(-rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.cos(-rotation + 2.0 / 3.0 * Math.PI) * size,
    );
    this.lineTo(
        x + Math.sin(-rotation + 4.0 / 3.0 * Math.PI) * size,
        y + Math.cos(-rotation + 4.0 / 3.0 * Math.PI) * size,
    );
    this.lineTo(
        x + Math.sin(-rotation) * size * 1.5,
        y + Math.cos(-rotation) * size * 1.5,
    );

    this.stroke();
}

for (const bird of simulation.world().birds) {
    ctxt.drawTriangle(
        bird.x * viewportWidth,
        bird.y * viewportHeight, 
        0.01 * viewportWidth,
        bird.rotation);
}