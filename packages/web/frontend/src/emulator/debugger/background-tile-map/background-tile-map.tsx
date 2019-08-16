import React, {FunctionComponent, useRef, useEffect} from 'react';
import { DebugInfo } from 'rustyboy-web';
import './background-tile-map.css';

interface Props {
    debugInfo?: DebugInfo;
}

function drawCanvas(buffer: Uint8Array, dimensions: [number, number], canvasRef: HTMLCanvasElement) {
  const context = canvasRef.getContext('2d');
  if (!context) return;
  context.clearRect(0, 0, canvasRef.width, canvasRef.height);
  const imageData = context.createImageData(dimensions[0], dimensions[1]);

  for (let i = 0; i < dimensions[0] * dimensions[1]; i++) {
    const bufferIndex = i * 3;
    const imageDataIndex = i * 4;
    imageData.data[imageDataIndex] = buffer[bufferIndex];
    imageData.data[imageDataIndex + 1] = buffer[bufferIndex + 1];
    imageData.data[imageDataIndex + 2] = buffer[bufferIndex + 2];
    imageData.data[imageDataIndex + 3] = 255;
  }

  context.putImageData(imageData, 0, 0);
}

export const BackgroundTileMap: FunctionComponent<Props> = ({debugInfo}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    if (debugInfo && canvasRef.current) {
      drawCanvas(debugInfo.background(), [256, 256], canvasRef.current);
    }
  }, [canvasRef, debugInfo]);

  return (
    <div className="background-tile-map">
      <canvas width={256} height={256} ref={canvasRef} />
    </div>
  );
};

export default BackgroundTileMap;