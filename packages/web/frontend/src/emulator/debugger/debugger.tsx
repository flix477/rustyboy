import React, { FunctionComponent, useState, useEffect, CSSProperties } from 'react';
import { Debugger as DebuggerType } from 'rustyboy-web';
import MemoryMap from './memory-map';

interface Props {
  loaded: boolean
}

function style(loaded: boolean): CSSProperties {
  if (!loaded) return {
    opacity: 0.2,
    pointerEvents: 'none'
  };

  return {};
}

export const Debugger: FunctionComponent<Props> = ({loaded}) => {
  const [debuggerRef, setDebuggerRef] = useState<DebuggerType>();

  useEffect(() => {
    setDebuggerRef(new DebuggerType());
  }, [debuggerRef]);

  return (
    <div style={style(loaded)}>
      <MemoryMap />
    </div>
  );
};

export default Debugger;