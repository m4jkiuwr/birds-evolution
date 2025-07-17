import * as sim from "lib-simulation-wasm";

CanvasRenderingContext2D.prototype.drawTriangle =
  function (x, y, size, rotation, innerAngle = 5.0 / 6.0) {
    const PI = Math.PI;
    const a = [
      x + size * Math.sin(rotation + PI),
      y - size * Math.cos(rotation + PI)
    ]
    const b = [
      x + size * Math.sin(innerAngle * PI + rotation + PI),
      y - size * Math.cos(innerAngle * PI + rotation + PI)
    ]
    const c = [
      x + size * Math.sin(- innerAngle * PI + rotation + PI),
      y - size * Math.cos(- innerAngle * PI + rotation + PI)
    ]
    this.beginPath();

    this.moveTo(...a);
    this.lineTo(...b);
    this.lineTo(...c);
    this.lineTo(...a);

    // this.strokeStyle = 'rgb(0,0,0)';
    this.fillStyle = 'rgba(217, 226, 246, 1)';
    this.fill();

  }

CanvasRenderingContext2D.prototype.drawCircle =
  function (x, y, size) {
    this.beginPath();
    this.arc(x, y, size, 0, 2.0 * Math.PI);
    this.fillStyle = 'rgba(72, 178, 100, 1)';
    this.fill();
  }

const simulation = new sim.Simulation(30, 80);

function reDraw() {
  ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
  simulation.step();
  const world = simulation.world();

  for (const animal of world.animals) {
    ctxt.drawTriangle(animal.x * viewportWidth,
      animal.y * viewportHeight,
      0.015 * viewportWidth,
      animal.rotation,
      4.7 / 6.0);
  }
  for (const food of world.foods) {
    ctxt.drawCircle(food.x * viewportWidth, food.y * viewportHeight, 0.005 * viewportWidth);
  }

  requestAnimationFrame(reDraw);
}




const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const ctxt = viewport.getContext("2d");
ctxt.scale(viewportScale, viewportScale);

reDraw();
