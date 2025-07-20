import React from 'react';

interface OverlayProps {
  onClick: () => void;
}
const PlaySVG = () => {
  return (
    <svg
      className="w-20 h-20 fill-white drop-shadow-lg"
      viewBox="0 0 24 24"
    >
      <path d="M8 5v14l11-7z" />
    </svg>
  )
}
const PauseSVG = () => {
  return (
    <svg
      className="w-20 h-20 fill-white drop-shadow-lg"
      viewBox="0 0 24 24"
    >
      <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
    </svg>
  );
}



const PauseOverlay: React.FC<OverlayProps> = ({ onClick }) => {
  return (
    <div
      className="
        absolute inset-0 
        bg-black/50 
        flex justify-center items-center 
        cursor-pointer
      "
      onClick={onClick}
    ><PlaySVG /></div>
  );
}

const HoverOverlay: React.FC<OverlayProps> = ({ onClick }) => {

  return (
    <div
      className="
        absolute inset-0 
        bg-black/50 
        flex justify-center items-center 
        cursor-pointer
        opacity-0 hover:opacity-50
        transition-opacity duration-100
      "
      onClick={onClick}
    ><PauseSVG /></div>
  );
}





export { PauseOverlay, HoverOverlay };