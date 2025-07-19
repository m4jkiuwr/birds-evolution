import React from "react";

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



export default SpeedSlider;

