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
  const bgTileMap1Ref = useRef<HTMLCanvasElement>(null);
  const bgTileMap2Ref = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (debugInfo && bgTileMap1Ref.current) {
      drawCanvas(debugInfo.background_tile_map1(), [256, 256], bgTileMap1Ref.current);
    }
  }, [bgTileMap1Ref, debugInfo]);

  useEffect(() => {
    if (debugInfo && bgTileMap2Ref.current) {
      drawCanvas(debugInfo.background_tile_map2(), [256, 256], bgTileMap2Ref.current);
    }
  }, [bgTileMap2Ref, debugInfo]);

  return (
    <div className="background-tile-map">
      <canvas width={256} height={256} ref={bgTileMap1Ref} />
      <canvas width={256} height={256} ref={bgTileMap2Ref} />
    </div>
  );
};

export default BackgroundTileMap;