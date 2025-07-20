import React from "react";
import { Play, Pause, FastForward } from "lucide-react";

const IconSize: number = 20;

interface SpeedSliderProps {
  sliderChange: (x: React.ChangeEvent<HTMLInputElement>) => void;
  currentSpeed: number
}
const SpeedSlider: React.FC<SpeedSliderProps> = ({ sliderChange, currentSpeed }) => {

  return (
    <div className="items-center gap-2">
      <label htmlFor="speed-slider" className="text-sm whitespace-nowrap">Sim Speed: {currentSpeed}</label>
      <input
        id="speed-slider"
        type="range"
        min="1"
        max="10"
        step="1"
        value={currentSpeed}
        onChange={sliderChange}
        className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
      />
    </div>
  );
}

interface PlayButtonProps {
  onPlayButtonClick: () => void;
  isPlaying: boolean;
}
const PlayButton: React.FC<PlayButtonProps> = ({ onPlayButtonClick, isPlaying }) => {
  return (
    <div>
      <button
        onClick={onPlayButtonClick}
      >
        {isPlaying ? <Pause size={IconSize} /> : <Play size={IconSize} />}
      </button>
    </div>
  );
}

interface TrainButtonProps {
  onTrainButtonClick: () => void;
}
const TrainButton: React.FC<TrainButtonProps> = ({ onTrainButtonClick }) => {
  return (
    <div>
      <button
        onClick={onTrainButtonClick}
      >
        <FastForward size={IconSize} />
      </button>
    </div>
  );

}



interface LiveInfoProps {
  minScore: number,
  maxScore: number,
  avgScore: number,
}
const LiveInfo: React.FC<LiveInfoProps> = ({ minScore, maxScore, avgScore }) => {
  return (
    <div className="grid grid-cols-3 gap-2 text-center p-2 bg-gray-700/50 rounded-lg">
      <div>
        <div className="text-xs text-gray-400">MIN</div>
        <div className="font-mono text-lg">{minScore}</div>
      </div>
      <div>
        <div className="text-xs text-gray-400">AVG</div>
        <div className="font-mono text-lg">{avgScore}</div>
      </div>
      <div>
        <div className="text-xs text-gray-400">MAX</div>
        <div className="font-mono text-lg">{maxScore}</div>
      </div>
    </div>
  );
}

interface GeneralProps {
  populationCount: number,
  foodCount: number,
  generation: number,
}
const GeneralInfoDisplay: React.FC<GeneralProps> = ({ generation, populationCount, foodCount }) => {
  return (
    <div className="flex flex-col gap-2 text-sm">
      <div className="flex justify-between items-center">
        <span className="font-semibold text-gray-300">Generation</span>
        <span className="font-mono bg-gray-700 px-2 py-0.5 rounded">{generation}</span>
      </div>
      <div className="flex justify-between items-center">
        <span className="font-semibold text-gray-300">Population</span>
        <span className="font-mono bg-gray-700 px-2 py-0.5 rounded">{populationCount}</span>
      </div>
      <div className="flex justify-between items-center">
        <span className="font-semibold text-gray-300">Food</span>
        <span className="font-mono bg-gray-700 px-2 py-0.5 rounded">{foodCount}</span>
      </div>
    </div>
  );
}

type GenerationInfoProps = SpeedSliderProps & PlayButtonProps & LiveInfoProps & GeneralProps & TrainButtonProps;

const GenerationInfo: React.FC<GenerationInfoProps> = ({
  sliderChange,
  currentSpeed,
  onPlayButtonClick,
  isPlaying,
  minScore,
  avgScore,
  maxScore,
  populationCount,
  foodCount,
  generation,
  onTrainButtonClick
}) => {
  return (
    <div className="flex flex-col gap-4 p-6 bg-gray-800 text-white rounded-lg shadow-xl">
      <h2 className="text-2xl font-bold text-center border-b border-gray-600 pb-3">
        Simulation
      </h2>

      <GeneralInfoDisplay
        generation={generation}
        populationCount={populationCount}
        foodCount={foodCount}
      />

      <h3 className="text-lg font-bold text-center border-b border-gray-700 pb-2 pt-2">
        Live Stats
      </h3>
      <LiveInfo minScore={minScore} maxScore={maxScore} avgScore={avgScore} />

      <h3 className="text-lg font-bold text-center border-b border-gray-700 pb-2 pt-2">
        Controls
      </h3>
      <SpeedSlider
        currentSpeed={currentSpeed}
        sliderChange={sliderChange}
      />

      <div className="flex justify-center gap-4">
        <PlayButton
          isPlaying={isPlaying}
          onPlayButtonClick={onPlayButtonClick}
        />
        <TrainButton
          onTrainButtonClick={onTrainButtonClick}
        />
      </div>
    </div>
  );
};

export default GenerationInfo;

