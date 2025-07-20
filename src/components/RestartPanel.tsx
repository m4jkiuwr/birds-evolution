import React, { useState } from "react"
import { RotateCcw } from "lucide-react";

interface RestartPanelProps {
  onClick: (populationCnt: number, animalCnt: number) => void;
  initPopulationCnt: number;
  initFoodCnt: number;
}

const RestartPanel: React.FC<RestartPanelProps> = ({ onClick, initPopulationCnt, initFoodCnt }) => {
  const [food, setFood] = useState<number>(initFoodCnt);
  const [population, setPopulation] = useState<number>(initPopulationCnt);

  const handleClick = () => onClick(population, food)

  return (
    <div className="grid grid-cols-[3fr_1fr] gap-4 p-4 bg-gray-800 rounded-lg">
      <div className="flex flex-col gap-4">
        <div className="flex justify-between items-center text-sm">
          <label htmlFor="population-slider">Population</label>
          <span className="font-mono bg-gray-700 px-2 py-0.5 rounded">{population}</span>
        </div>
        <input
          id="population-slider"
          type="range"
          min="1"
          max="100"
          value={population}
          onChange={(e) => setPopulation(Number(e.target.value))}
          className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
        />


        <div className="flex justify-between items-center text-sm">
          <label htmlFor="population-slider">Food</label>
          <span className="font-mono bg-gray-700 px-2 py-0.5 rounded">{food}</span>
        </div>
        <input
          id="population-slider"
          type="range"
          min="1"
          max="100"
          value={food}
          onChange={(e) => setFood(Number(e.target.value))}
          className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
        />
      </div>
      <div className="flex items-center justify-center">

        <button
          onClick={handleClick}
          className="p-2"
        ><RotateCcw size={20} />
        </button>
      </div>
    </div>






  );
}

export default RestartPanel;