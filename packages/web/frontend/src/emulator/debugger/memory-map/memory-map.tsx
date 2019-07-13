import React, { FunctionComponent, useState, useEffect, useRef } from 'react';
import { FixedSizeList as List } from 'react-window';
import AutoSizer from 'react-virtualized-auto-sizer';
import Line, {Instruction} from './line';
import "./memory-map.css";

const initialInstructions = Array.from({length: 0x10000}, (_, i) => ({
  line: i,
  mnemonic: "NOP",
  operands: "nn,n"
}));

interface Props {
  instructions?: Instruction[];
  currentLine?: number;
}

export const MemoryMap: FunctionComponent<Props> = ({instructions, currentLine}) => {
  const listRef = useRef<List>(null);
  const [lastInstructions, setLastInstructions] = useState<Instruction[]>(initialInstructions);
  const data = {
    currentLine,
    instructions: lastInstructions
  };

  useEffect(() => {
    if (instructions) {
      setLastInstructions(instructions);
    }
  }, [instructions]);

  useEffect(() => {
    if (currentLine !== undefined && listRef.current) {
      const index = lastInstructions.findIndex(instruction => instruction.line === currentLine);
      if (index !== -1) listRef.current.scrollToItem(index);
    }
  }, [currentLine, lastInstructions]);

  return (
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
  );
};

export default MemoryMap;