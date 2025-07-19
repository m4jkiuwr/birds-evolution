import React from "react";
import { Play, Pause } from "lucide-react";

const IconSize: number = 20;

interface SpeedSliderProps {
  onChange: (x: React.ChangeEvent<HTMLInputElement>) => void;
  currentSpeed: number
}
const SpeedSlider: React.FC<SpeedSliderProps> = ({ onChange, currentSpeed }) => {

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
        onChange={onChange}
        className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
      />
    </div>
  );
}
interface PlayButtonProps {
  onClick: () => void;
  isPlaying: boolean;
}
const PlayButton: React.FC<PlayButtonProps> = ({ onClick, isPlaying }) => {
  return (
    <div>
      <button
        onClick={onClick}
      >
        {isPlaying ? <Pause size={IconSize} /> : <Play size={IconSize} />}
      </button>
    </div>
  );


}



export { SpeedSlider, PlayButton };

