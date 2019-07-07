import React, { FunctionComponent, CSSProperties, useCallback, useState, useEffect, useMemo } from 'react';
import { Debugger as DebuggerType, DebugInfo, RegisterTypeJs } from 'rustyboy-web';
import MemoryMap from './memory-map';
import { Instruction } from './memory-map/memory-map';

interface Props {
  debuggerRef: DebuggerType;
  debugInfo?: DebugInfo;
}

function style(loaded: boolean): CSSProperties {
  if (!loaded) return {
    opacity: 0.2
  };

  return {};
}

export const Debugger: FunctionComponent<Props> = ({debuggerRef, debugInfo}) => {
  const [lastDebugInfo, setLastDebugInfo] = useState<DebugInfo>();
  const [instructions, setInstructions] = useState<Instruction[]>();
  const loaded = Boolean(debugInfo);

  useEffect(() => {
    if (debugInfo) {
      setLastDebugInfo(debugInfo);
      setInstructions(debugInfo.parseAll());
    }
  }, [debugInfo])

  const onClick = useCallback(() => {
    if (!loaded) {
      debuggerRef.addBreakpoint(RegisterTypeJs.PC, 0x40);
    }
  }, [debuggerRef, loaded]);

  return (
    <div style={style(loaded)} onClick={onClick}>
      <MemoryMap instructions={instructions} />
    </div>
  );
};

export default Debugger;