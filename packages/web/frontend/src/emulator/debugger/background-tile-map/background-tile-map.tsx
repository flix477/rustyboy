import React, {FunctionComponent, useRef, useEffect} from 'react';
import { DebugInfo } from 'rustyboy-web';

interface Props {
    debugInfo?: DebugInfo;
}

function drawCanvas(buffer: Uint8Array, dimensions: [number, number], canvasRef: HTMLCanvasElement) {
  const context = canvasRef.getContext('2d');
  if (!context) return;
  context.clearRect(0, 0, canvasRef.width, canvasRef.height);
  const imageData = context.createImageData(dimensions[0], dimensions[1]);

  for (let i = 0; i < imageData.data.length; i++) {
    imageData.data[i] = buffer[i];
    imageData.data[i + 1] = buffer[i + 1];
    imageData.data[i + 2] = buffer[i + 2];
    imageData.data[i + 3] = buffer[i + 3];
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
      <canvas ref={canvasRef} />
    </div>
  );
};

export default BackgroundTileMap;