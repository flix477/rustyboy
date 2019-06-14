import React, {FunctionComponent, useEffect, useCallback} from 'react';
import {Gameboy as GameboyType, InputButton, InputTypeJs, Input} from 'rustyboy-web';

function update(gameboy: GameboyType) {
  gameboy.run_to_vblank();
  requestAnimationFrame(() => update(gameboy));
}

function onInput(gameboy: GameboyType): EventListener {
  return event => {
    const input = eventToInput(event as KeyboardEvent);
    if (!input) return;
    gameboy.send_input(input);
  }
}

function eventToInput(event: KeyboardEvent): Input | null {
  const button = eventToInputButton(event);
  const type = eventToInputType(event);
  if (!button || !type) return null;
  console.log(button, type);
  // @ts-ignore
  return new Input("start", "down");
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
  const inputCallback = useCallback(onInput(gameboy), [gameboy]);

  useEffect(() => {
    update(gameboy);
    window.addEventListener("keydown", inputCallback);
    window.addEventListener("keyup", inputCallback);

    return () => {
      window.removeEventListener("keydown", inputCallback);
      window.removeEventListener("keyup", inputCallback);
    };
  }, [gameboy, inputCallback]);

  return (
    <canvas id="canvas" />
  );
}

export default Gameboy;