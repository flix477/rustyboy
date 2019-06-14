import React, {FunctionComponent, useEffect, useCallback, useRef, RefObject} from 'react';
import {Gameboy as GameboyType, InputButton, InputTypeJs, Input, getScreenWidth, getScreenHeight} from 'rustyboy-web';

async function update(gameboy: GameboyType, canvasRef: RefObject<HTMLCanvasElement>) {
  if (!canvasRef.current) return;
  const canvas = canvasRef.current;
  const context = canvas.getContext('2d');
  if (!context) return;
  context.imageSmoothingEnabled = false;

  const buffer = gameboy.runToVBlank();
  const array = new Uint8ClampedArray(buffer);
  const imageData = new ImageData(array, getScreenWidth(), getScreenHeight());
  const imageBitmap = await window.createImageBitmap(imageData);

  context.drawImage(imageBitmap, 0, 0, canvas.width, canvas.height);
  
  requestAnimationFrame(() => update(gameboy, canvasRef));
}

function onInput(gameboy: GameboyType): EventListener {
  return event => {
    const input = eventToInput(event as KeyboardEvent);
    if (!input) return;
    gameboy.sendInput(input);
  }
}

function eventToInput(event: KeyboardEvent): Input | null {
  const button = eventToInputButton(event);
  const type = eventToInputType(event);
  if (button === null || type === null) return null;
  return new Input(type, button);
}

function eventToInputButton(event: KeyboardEvent): InputButton | null {
  switch (event.key) {
    case "z": return InputButton.A;
    case "x": return InputButton.B;
    case "Enter": return InputButton.Start;
    case " ": return InputButton.Select;
    case "ArrowLeft": return InputButton.Left;
    case "ArrowRight": return InputButton.Right;
    case "ArrowUp": return InputButton.Up;
    case "ArrowDown": return InputButton.Down;
    default: return null;
  }
}

function eventToInputType(event: KeyboardEvent): InputTypeJs | null {
  switch (event.type) {
    case "keydown": return InputTypeJs.Down;
    case "keyup": return InputTypeJs.Up;
    default: return null;
  }
}

interface Props {
  gameboy: GameboyType
}

const Gameboy: FunctionComponent<Props> = ({gameboy}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const inputCallback = useCallback(onInput(gameboy), [gameboy]);

  useEffect(() => {
    update(gameboy, canvasRef);
    window.addEventListener("keydown", inputCallback);
    window.addEventListener("keyup", inputCallback);

    return () => {
      window.removeEventListener("keydown", inputCallback);
      window.removeEventListener("keyup", inputCallback);
    };
  }, [gameboy, inputCallback, canvasRef]);

  return (
    <div>
      <canvas width="320" height="288" ref={canvasRef} id="canvas" />
    </div>
  );
}

export default Gameboy;