import React, { useEffect, useRef, useState } from 'react';
import { Simulation, World, Animal, Food, Statistics } from '../simulation-wasm'
import { PauseOverlay, HoverOverlay } from './PauseOverlay';
import GenerationInfo from './GenerationInfo';
import HistoryChart from './HistoryChart';

const BIRD_SIZE: number = 0.02;
const FOOD_SIZE: number = 0.005;

const POPULATION_COUNT: number = 40;
const FOOD_COUNT: number = 60;

const BIRD_ANGLE: number = 4.8 / 6.0;


type SimType = InstanceType<typeof Simulation>
type AnimalType = typeof Animal.prototype;
type FoodType = typeof Food.prototype;
type WorldType = typeof World.prototype;
type StatType = Omit<typeof Statistics.prototype, 'free'>;

declare global {
  interface CanvasRenderingContext2D {
    drawBird(bird: AnimalType, size: number, width: number, height: number): void,
    drawFood(food: FoodType, size: number, width: number, height: number): void
  }
}
CanvasRenderingContext2D.prototype.drawBird = function (bird: AnimalType, size: number, width: number, height: number) {
  const [x, y, rotation] = [bird.x * width, bird.y * height, bird.rotation];
  const innerAngle = BIRD_ANGLE;
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

function calcStats(world: WorldType | null): StatType {
  if (!world || world.animals.length === 0) {
    return {
      min_score: 0, max_score: 0, avg_score: 0
    }
  } else {
    const scores = world.animals.map((value) => value.score);

    const minScore = Math.min(...scores);
    const maxScore = Math.max(...scores);
    const avgScore = scores.reduce((sum, score) => sum + score, 0) / scores.length;

    return {
      min_score: minScore,
      max_score: maxScore,
      avg_score: parseFloat(avgScore.toFixed(2))
    }
  }
}

function runSimulationStep(sim: SimType, speed: number): StatType[] {
  const newStats: StatType[] = [];
  for (let i = 0; i < speed; i++) {
    const currStats = sim.step();
    if (currStats !== undefined) newStats.push(currStats);
  }
  return newStats;

}





const SimCanvas: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const simRef = useRef<SimType | null>(null);
  const [world, setWorld] = useState<WorldType | null>(null);
  const [isPlaying, setIsPlaying] = useState<boolean>(true);
  const [simSpeed, setSimSpeed] = useState<number>(1);
  const [statsHistory, setStatsHistory] = useState<StatType[]>([]);


  useEffect(() => {
    simRef.current = startSimulation({ animalCount: POPULATION_COUNT, foodCount: FOOD_COUNT });
    setWorld(simRef.current.world());
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
    if (!isPlaying || simRef.current === null) {
      return;
    } else {
      const simulation = simRef.current;
      let animationFrameId: number;

      const gameLoop = () => {
        const genStats: StatType[] = runSimulationStep(simulation, simSpeed);

        setWorld(simulation.world());
        if (genStats.length > 0) {
          setStatsHistory(prevHistory => [...prevHistory, ...genStats]);
        }
        animationFrameId = window.requestAnimationFrame(gameLoop);
      };

      gameLoop();

      return () => window.cancelAnimationFrame(animationFrameId);
    }
  }, [isPlaying, simRef.current, simSpeed]);

  const handleSimClick = () => {
    setIsPlaying(!isPlaying);
  }
  const handleSliderChange = (x: React.ChangeEvent<HTMLInputElement>) => {
    const newSpeed = Number(x.target.value);
    setSimSpeed(newSpeed);
  }
  const handleTrainClick = () => {
    if (!simRef.current) {
      return;
    } else {
      const newStats: StatType = simRef.current.train();
      setStatsHistory([...statsHistory, newStats]);
    }
  }


  const currStats = calcStats(world);

  return (
    <div className="flex flex-row items-start justify-center gap-8 p-8 bg-gray-900 min-h-screen">
      <div className="relative w-[800px] h-[800px] border border-white">
        <canvas
          ref={canvasRef}
          className="w-full h-full"
          onClick={handleSimClick}
        />
        {isPlaying ?
          <HoverOverlay onClick={handleSimClick} /> :
          <PauseOverlay onClick={handleSimClick} />
        }
      </div>
      <div className="gap-6">
        <GenerationInfo
          currentSpeed={simSpeed}
          isPlaying={isPlaying}
          sliderChange={handleSliderChange}
          onPlayButtonClick={handleSimClick}
          minScore={currStats.min_score}
          avgScore={currStats.avg_score}
          maxScore={currStats.max_score}
          generation={statsHistory.length + 1}
          foodCount={FOOD_COUNT}
          populationCount={POPULATION_COUNT}
          onTrainButtonClick={handleTrainClick}
        />
        <HistoryChart statsHistory={statsHistory} />
      </div>
    </div>
  )

}





export default SimCanvas;