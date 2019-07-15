import React, { FunctionComponent, useEffect, useState, useCallback } from 'react';
import { Gameboy as GameboyType, Debugger as DebuggerType, DebugInfo } from 'rustyboy-web';

import Gameboy from './gameboy';
import Debugger from './debugger';
import "./emulator.css";

interface Props {
  gameboy: GameboyType
}

export const Emulator: FunctionComponent<Props> = ({ gameboy }) => {
  const [debuggerRef, setDebuggerRef] = useState<DebuggerType>();
  const [debugInfo, setDebugInfo] = useState<DebugInfo>();

  const onBreakpointHit = (debugInfo: DebugInfo) => {
    setDebugInfo(debugInfo);
  };

  useEffect(() => {
    setDebuggerRef(new DebuggerType());
  }, [setDebuggerRef]);

  const onContinue = useCallback(() => {
    setDebugInfo(undefined);
  }, [setDebugInfo]);

  return (
    <div className="emulator">
      <Gameboy
        gameboy={gameboy}
        debuggerRef={debuggerRef}
        onBreakpointHit={onBreakpointHit}
        paused={Boolean(debugInfo)}
        onClick={onContinue}
      />
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