import React, { FunctionComponent, useState, useEffect, useRef } from 'react';
import { FixedSizeList as List } from 'react-window';
import AutoSizer from 'react-virtualized-auto-sizer';
import Line, {Instruction} from './line';
import './memory-map.css';
import { DebugInfo } from 'rustyboy-web';

const initialInstructions = Array.from({length: 0x10000}, (_, i) => ({
  line: i,
  mnemonic: 'NOP',
  operands: 'nn,n'
}));

interface Props {
  debugInfo?: DebugInfo;
}

export const MemoryMap: FunctionComponent<Props> = ({debugInfo}) => {
  const listRef = useRef<List>(null);
  const currentLine = debugInfo && debugInfo.currentLine();
  const [lastInstructions, setLastInstructions] = useState<Instruction[]>(initialInstructions);
  const data = {
    currentLine,
    instructions: lastInstructions
  };

  useEffect(() => {
    if (debugInfo) {
      setLastInstructions(debugInfo.parseAll());
    }
  }, [debugInfo]);

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