import React, { FunctionComponent, useEffect, useState, useCallback } from 'react';
import { Gameboy as GameboyType, Debugger as DebuggerType, DebugInfo } from 'rustyboy-web';

import Gameboy from './gameboy';
import Debugger from './debugger';
import './emulator.css';

interface Props {
  gameboy: GameboyType;
  hasDebugger: boolean;
}

export const Emulator: FunctionComponent<Props> = ({ gameboy, hasDebugger }) => {
  const [debuggerRef, setDebuggerRef] = useState<DebuggerType>();
  const [debugInfo, setDebugInfo] = useState<DebugInfo>();

  const onBreakpointHit = (debugInfo: DebugInfo) => {
    setDebugInfo(debugInfo);
  };

  useEffect(() => {
    if (hasDebugger) {
      setDebuggerRef(new DebuggerType());
    }
  }, [hasDebugger, setDebuggerRef]);

  const onContinue = useCallback(() => {
    setDebugInfo(undefined);
  }, [setDebugInfo]);

  const onGameboyClick = useCallback(() => {
    if (debuggerRef) debuggerRef.continueExecution();
    onContinue();
  }, [debuggerRef, onContinue]);

  return (
    <div className="emulator">
      <div className="gameboyContainer">
        <Gameboy
          gameboy={gameboy}
          debuggerRef={debuggerRef}
          onBreakpointHit={onBreakpointHit}
          paused={Boolean(debugInfo)}
          onClick={onGameboyClick}
        />
      </div>
      {debuggerRef && (
        <Debugger
          debuggerRef={debuggerRef}
          debugInfo={debugInfo}
          onContinue={onContinue}
        />
      )}
    </div>
  );
};

export default Emulator;