import React, { FunctionComponent, useState, useEffect, useRef, useCallback } from 'react';
import { FixedSizeList as List } from 'react-window';
import AutoSizer from 'react-virtualized-auto-sizer';
import Line from './line';
import { DebugInfo } from 'rustyboy-web';
import './memory-map.css';

const initialInstructions = Array.from({length: 0x10000}, (_, i) => ({
  line: i,
  mnemonic: 'NOP',
  operands: 'nn,n'
}));

interface Props {
  debugInfo?: DebugInfo;
  addBreakpoint: (line: number) => void;
  removeBreakpoint: (line: number) => void;
  breakpoints: Uint16Array;
}

interface Instruction {
  line: number;
  mnemonic: string;
  operands: string;
}

export const MemoryMap: FunctionComponent<Props> = ({debugInfo, addBreakpoint, removeBreakpoint, breakpoints}) => {
  const listRef = useRef<List>(null);
  const currentLine = debugInfo && debugInfo.currentLine();
  const [lastInstructions, setLastInstructions] = useState<Instruction[]>(initialInstructions);
  const data = {
    currentLine,
    instructions: lastInstructions
  };

  const onBreakpoint = useCallback(line => {
    const breakpoint = breakpoints.findIndex(x => x === line);
    if (breakpoint !== -1) {
      removeBreakpoint(breakpoint);
    } else {
      addBreakpoint(line);
    }
  }, [breakpoints, addBreakpoint, removeBreakpoint]);

  useEffect(() => {
    if (debugInfo) {
      setLastInstructions(
        debugInfo.parseAll()
          .map((instruction: Instruction) => ({
            ...instruction,
            isBreakpoint: breakpoints.some(x => instruction.line === x),
            onBreakpoint
          }))
      );
    }
  }, [debugInfo, breakpoints, onBreakpoint]);

  useEffect(() => {
    if (currentLine !== undefined && listRef.current) {
      const index = lastInstructions.findIndex(instruction => instruction.line === currentLine);
      if (index !== -1) listRef.current.scrollToItem(index, 'center');
    }
  }, [currentLine, lastInstructions]);

  return (
    <div>
      <AutoSizer>
        {({width, height}) => (
          <List
            ref={listRef}
            className="memory-map"
            itemSize={20}
            height={height}
            width={width}
            itemCount={lastInstructions.length}
            itemData={data}
          >
            {Line}
          </List>
        )}
      </AutoSizer>
    </div>
  );
};

export default MemoryMap;