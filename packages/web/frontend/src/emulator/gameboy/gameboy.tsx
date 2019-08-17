import React, {FunctionComponent, useEffect, useCallback} from 'react';
import {Gameboy as GameboyType, InputButton, InputTypeJs, Input, Debugger, DebugInfo} from 'rustyboy-web';

import {ReactComponent as GameboyBackground} from './gameboy-full-o1.svg';
import './gameboy.css';

function className(paused: boolean): string {
  return paused ? 'gameboy disabled' : 'gameboy';
}

function eventToInputButton(event: KeyboardEvent): InputButton | null {
  switch (event.key) {
  case 'z': return InputButton.A;
  case 'x': return InputButton.B;
  case 'Enter': return InputButton.Start;
  case ' ': return InputButton.Select;
  case 'ArrowLeft': return InputButton.Left;
  case 'ArrowRight': return InputButton.Right;
  case 'ArrowUp': return InputButton.Up;
  case 'ArrowDown': return InputButton.Down;
  default: return null;
  }
}

function eventToInputType(event: KeyboardEvent): InputTypeJs | null {
  switch (event.type) {
  case 'keydown': return InputTypeJs.Down;
  case 'keyup': return InputTypeJs.Up;
  default: return null;
  }
}

function eventToInput(event: KeyboardEvent): Input | null {
  const button = eventToInputButton(event);
  const type = eventToInputType(event);
  if (button === null || type === null) return null;
  return new Input(type, button);
}

function onInput(gameboy: GameboyType): EventListener {
  return event => {
    const input = eventToInput(event as KeyboardEvent);
    if (!input) return;
    gameboy.sendInput(input);
  };
}

function update(gameboy: GameboyType, debuggerRef?: Debugger, onBreakpointHit?: (debugInfo: DebugInfo) => void) {
  return () => {
    let debugInfo = undefined;
    if (debuggerRef) {
      debugInfo = gameboy.runToEvent(debuggerRef);
    } else gameboy.runToVBlank();
    
    if (debugInfo && onBreakpointHit) {
      onBreakpointHit(debugInfo);
      return false;
    }

    return true;
  };
}

function useUpdate(updateFn: () => boolean, condition?: boolean) {
  useEffect(() => {
    if (!condition) return;
    let handle: number | null = null;

    function callback() {
      if (updateFn())
        handle = requestAnimationFrame(callback);
    }

    callback();

    return () => {
      if (handle !== null) cancelAnimationFrame(handle);
    };
  }, [updateFn, condition]);
}

interface Props {
  gameboy: GameboyType;
  debuggerRef?: Debugger;
  onBreakpointHit?: (debugInfo: DebugInfo) => void;
  paused: boolean;
  onClick?: () => void;
}

const Gameboy: FunctionComponent<Props> = ({gameboy, debuggerRef, onBreakpointHit, paused, onClick}) => {
  const inputCallback = useCallback(onInput(gameboy), [gameboy]);
  const updateCallback = useCallback(update(gameboy, debuggerRef, onBreakpointHit), [gameboy, debuggerRef, onBreakpointHit]);
  useUpdate(updateCallback, !paused);

  useEffect(() => {
    window.addEventListener('keydown', inputCallback);
    window.addEventListener('keyup', inputCallback);

    return () => {
      window.removeEventListener('keydown', inputCallback);
      window.removeEventListener('keyup', inputCallback);
    };
  }, [inputCallback]);

  return (
    <div className={className(paused)} onClick={onClick}>
      <GameboyBackground className="background" />
      <canvas width="345" height="313" id="canvas" />
    </div>
  );
};

export default Gameboy;