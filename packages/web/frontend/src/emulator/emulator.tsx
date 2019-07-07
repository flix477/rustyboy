import React, { FunctionComponent, useEffect, useState } from 'react';
import { Gameboy as GameboyType, Debugger as DebuggerType, DebugInfo } from 'rustyboy-web';

import Gameboy from './gameboy';
import Debugger from './debugger';

interface Props {
  gameboy: GameboyType
}

export const Emulator: FunctionComponent<Props> = ({ gameboy }) => {
  const [debuggerRef, setDebuggerRef] = useState<DebuggerType>();
  const [debugInfo, setDebugInfo] = useState<DebugInfo>();

  const onBreakpointHit = (debugInfo: DebugInfo) => {
    console.log("hi");
    setDebugInfo(debugInfo);
  };

  useEffect(() => {
    setDebuggerRef(new DebuggerType());
  }, [setDebuggerRef]);

  return (
    <div>
      <Gameboy gameboy={gameboy} debuggerRef={debuggerRef} onBreakpointHit={onBreakpointHit} />
      {debuggerRef && <Debugger debuggerRef={debuggerRef} debugInfo={debugInfo} />}
    </div>
  );
};

export default Emulator;