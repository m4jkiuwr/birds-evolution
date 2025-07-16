import * as sim from "lib-simulation-wasm";

const drawTriangle = (ctxt, x, y, size) => {
  ctxt.beginPath();
  ctxt.moveTo(x, y);
  ctxt.lineTo(x + size, y + size);
  ctxt.lineTo(x - size, y + size);
  ctxt.lineTo(x, y);

  ctxt.fillStyle = 'rgb(0,0,0)';
  ctxt.fill();
}





const simulation = new sim.Simulation();
const world = simulation.world();

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

ctxt.fillStyle = 'rgb(0,0,0)';

const getCoords = (animal) => {
  return [viewportWidth * animal.x, viewportHeight * animal.y, 10, 10]
}

drawTriangle(ctxt, 50, 0, 50);
