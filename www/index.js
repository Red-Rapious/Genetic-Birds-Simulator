import * as sim from "lib-simulation-wasm";

const STEPS_PER_FRAME = 1;

var simulation = new sim.Simulation();

const viewport = document.getElementById("viewport");

var simulationPaused = false;
const pauseCheckbox = document.getElementById("pause");
pauseCheckbox.checked = simulationPaused;

pauseCheckbox.onclick = function() {
    simulationPaused = pauseCheckbox.checked;
    if (!simulationPaused) {
        redraw();
    }
}

const restartBtn = document.getElementById("restart");
restartBtn.onclick = function() {
    simulation = new sim.Simulation();
}

// Maps the "Next Generation" button to `simulation.train()`
document.getElementById("train").onclick = function() {
    console.log(simulation.train());
}

// Adapat the viewport scale to avoid pixelized images.
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';


const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);

// Draws a simple white triangle used for birds
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

    this.fillStyle = 'rgb(255, 255, 255)'; // white
    this.fill();
}

/// Draws a simple green circle used for food
CanvasRenderingContext2D.prototype.drawCircle = function(x, y, radius) {
    this.beginPath();

    this.arc(x, y, radius, 0, 2.0 * Math.PI);

    this.fillStyle = 'rgb(0, 255, 128)'; // green-ish
    this.fill();
}

// Draws birds and foods on the viewport
function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

    const world = simulation.world();

    // Increases simulation speed
    for (let i = 0; i < STEPS_PER_FRAME; i += 1) {
        simulation.step();
    }

    for (const food of world.foods) {
        ctxt.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            (0.01 / 2.0) * viewportWidth
        );
    }

    for (const bird of world.birds) {
        ctxt.drawTriangle(
            bird.x * viewportWidth,
            bird.y * viewportHeight, 
            0.01 * viewportWidth,
            bird.rotation);
    }

    if (!simulationPaused) {
        requestAnimationFrame(redraw);
    }
}

redraw();