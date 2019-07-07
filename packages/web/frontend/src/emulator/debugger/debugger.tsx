import React, { FunctionComponent, CSSProperties, useCallback } from 'react';
import { Debugger as DebuggerType, DebugInfo, RegisterTypeJs } from 'rustyboy-web';
import MemoryMap from './memory-map';

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
  const loaded = Boolean(debugInfo);

  const onClick = useCallback(() => {
    if (!loaded) {
      debuggerRef.addBreakpoint(RegisterTypeJs.PC, 0x40);
    }
  }, [debuggerRef, loaded]);

  return (
    <div style={style(loaded)} onClick={onClick}>
      <MemoryMap bus={debugInfo && debugInfo.bus()} />
    </div>
  );
};

export default Debugger;