import React, { useEffect, useRef, useState } from 'react';
import { Simulation, World, Animal, Food, wasmReady } from '../wasm'

const BIRD_SIZE: number = 0.02;
const FOOD_SIZE: number = 0.005;


type SimType = InstanceType<typeof Simulation>
type AnimalType = typeof Animal.prototype;
type FoodType = typeof Food.prototype;
type WorldType = typeof World.prototype;

declare global {
  interface CanvasRenderingContext2D {
    drawBird(bird: AnimalType, size: number, width: number, height: number): void,
    drawFood(food: FoodType, size: number, width: number, height: number): void
  }

}
interface SimulationProps {
  animalCount: number,
  foodCount: number,

}

function startSimulation({ animalCount, foodCount }: SimulationProps): SimType {
  const sim = new Simulation(animalCount, foodCount);
  return sim;
}
function sharpenCanvas(canvas: HTMLCanvasElement): void {
  const ctxt = canvas.getContext('2d');
  if (!ctxt) return;

  const dpr = window.devicePixelRatio || 1;

  const rect = {
    width: canvas.clientWidth,
    height: canvas.clientHeight
  };

  if (canvas.width !== rect.width * dpr || canvas.height !== rect.height * dpr) {
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;

    ctxt.scale(dpr, dpr);
  }
}

CanvasRenderingContext2D.prototype.drawBird = function (bird: AnimalType, size: number, width: number, height: number) {
  const [x, y, rotation] = [bird.x * width, bird.y * height, bird.rotation];
  const innerAngle = 5.0 / 6.0;
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

  this.moveTo(a[0], a[1]);
  this.lineTo(b[0], b[1]);
  this.lineTo(c[0], c[1]);
  this.lineTo(a[0], a[1]);

}
CanvasRenderingContext2D.prototype.drawFood = function (food: FoodType, size: number, width: number, height: number) {
  const [x, y] = [food.x * width, food.y * height];
  this.moveTo(x, y);
  this.arc(x, y, size, 0, 2.0 * Math.PI);
}





const paintCanvas = (world: WorldType, canvasRef: React.RefObject<HTMLCanvasElement | null>) => {
  const canvas = canvasRef.current;
  if (!canvas) return;
  const ctxt = canvas.getContext('2d');
  if (!ctxt) return;

  // Use the CSS dimensions for all drawing calculations.
  const [w, h] = [canvas.clientWidth, canvas.clientHeight];

  ctxt.clearRect(0, 0, w, h);
  ctxt.fillStyle = 'rgba(217, 226, 246, 1)';
  ctxt.beginPath();
  world.animals.forEach((bird) => ctxt.drawBird(bird, BIRD_SIZE * w, w, h))
  ctxt.fill();

  ctxt.fillStyle = 'rgba(72, 178, 100, 1)';
  ctxt.beginPath();
  for (const food of world.foods) {
    ctxt.drawFood(food, FOOD_SIZE * w, w, h);
  }
  ctxt.fill();



}





const SimCanvas: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const simRef = useRef<SimType | null>(null);
  const [world, setWorld] = useState<WorldType | null>(null);
  const [isPlaying, setIsPlaying] = useState<boolean>(false);

  useEffect(() => {
    wasmReady.then(
      () => {
        simRef.current = startSimulation({ animalCount: 10, foodCount: 10 });
        setWorld(simRef.current.world());
      }
    )
  }, []);
  useEffect(() => {
    if (canvasRef.current) sharpenCanvas(canvasRef.current)
  }, [canvasRef.current])

  useEffect(() => {
    if (world) {
      paintCanvas(world, canvasRef);
    }
  }, [world]);

  useEffect(() => {
    if (!isPlaying || !simRef.current) return;
    let animationFrameId: number;

    const gameLoop = () => {
      simRef.current?.step();
      setWorld(simRef.current!.world());
      animationFrameId = window.requestAnimationFrame(gameLoop)
    };

    gameLoop();

    return () => window.cancelAnimationFrame(animationFrameId);
  }, [isPlaying]);

  const handleSimClick = () => {
    setIsPlaying(!isPlaying);
  }


  return (
    <>

      <canvas
        ref={canvasRef}
        style={{
          border: '1px solid white',
          width: '800px',
          height: '800px'
        }}
      />
      <button onClick={handleSimClick}>click</button>


    </>
  )

}





export default SimCanvas;