import React, { FunctionComponent, useCallback, useState, useEffect } from 'react';
import { Debugger as DebuggerType, DebugInfo, RegisterTypeJs } from 'rustyboy-web';

import Actions from './actions';
import MemoryMap from './memory-map';
import './debugger.css';

interface Props {
  debuggerRef: DebuggerType;
  debugInfo?: DebugInfo;
  onContinue?: () => void;
}

function className(loaded: boolean): string {
  return loaded ? 'debugger' : 'debugger disabled';
}

export const Debugger: FunctionComponent<Props> = ({debuggerRef, debugInfo, onContinue}) => {
  const [lastDebugInfo, setLastDebugInfo] = useState<DebugInfo>();
  const loaded = Boolean(debugInfo);

  useEffect(() => {
    if (debugInfo) {
      setLastDebugInfo(debugInfo);
    }
  }, [debugInfo]);

  const onClick = useCallback(() => {
    if (!loaded) {
      debuggerRef.addBreakpoint(RegisterTypeJs.PC, 0x40, true);
    }
  }, [debuggerRef, loaded]);

  return (
    <div className={className(loaded)} onClick={onClick}>
      <MemoryMap debugInfo={lastDebugInfo} />
      <Actions debugInfo={debugInfo} debuggerRef={debuggerRef} onContinue={onContinue} />
    </div>
  );
};

export default Debugger;