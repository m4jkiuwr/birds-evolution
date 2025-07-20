import React from "react";
import { Play, Pause } from "lucide-react";

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

interface LiveInfoProps {
  minScore: number,
  maxScore: number,
  avgScore: number,
}
const LiveInfo: React.FC<LiveInfoProps> = ({ minScore, maxScore, avgScore }) => {
  return (
    <>
      {minScore} | {avgScore} | {maxScore}
    </>
  );

}



type GenerationInfoProps = SpeedSliderProps & PlayButtonProps & LiveInfoProps;

const GenerationInfo: React.FC<GenerationInfoProps> = ({
  sliderChange,
  currentSpeed,
  onPlayButtonClick,
  isPlaying,
  minScore,
  avgScore,
  maxScore
}) => {
  return (
    <div className="flex flex-col gap-6 p-6 bg-gray-800 text-white rounded-lg shadow-xl w-[300px]">
      <h2 className="text-2xl font-bold text-center border-b border-gray-600 pb-3">
        Controls
      </h2>

      <div className="text-lg">
        <span className="font-semibold">Generation: </span>
        <span className="font-mono float-right bg-gray-700 px-2 rounded">2
        </span>
      </div>

      <SpeedSlider
        currentSpeed={currentSpeed}
        sliderChange={sliderChange}
      />

      <div className="flex justify-center pt-4">
        <PlayButton
          isPlaying={isPlaying}
          onPlayButtonClick={onPlayButtonClick}
        />
      </div>
      <div className="flex justify-center pt-4">
        <LiveInfo minScore={minScore} maxScore={maxScore} avgScore={avgScore} />
      </div>
    </div>
  );
};

export default GenerationInfo;

