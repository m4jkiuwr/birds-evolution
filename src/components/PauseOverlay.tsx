import React from 'react';
import { Play } from "lucide-react";

interface PauseOverlayProps {
  onClick: () => void;
}
const PauseSVG = () => {
  return (
    <svg
      className="w-20 h-20 fill-white drop-shadow-lg"
      viewBox="0 0 24 24"
    >
      <path d="M8 5v14l11-7z" />
    </svg>
  )
}


const PauseOverlay: React.FC<PauseOverlayProps> = ({ onClick }) => {
  return (
    <div
      className="
        absolute inset-0 
        bg-black/50 
        flex justify-center items-center 
        cursor-pointer
      "
      onClick={onClick}
    ><PauseSVG /></div>
  );
}

export default PauseOverlay;