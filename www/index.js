import * as sim from "lib-simulation-wasm";



const simulation = new sim.Simulation();
const world = simulation.world();

const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const ctxt = viewport.getContext("2d");
ctxt.fillStyle = 'rgb(0,0,0)';

const getCoords = (animal) => {
  return [viewportWidth * animal.x, viewportHeight * animal.y, 10, 10]
}



for (const animal of world.animals) {


  ctxt.fillRect(...getCoords(animal));

}
