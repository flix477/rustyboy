import React, { FunctionComponent, useState, useEffect } from 'react';
import { FixedSizeList as List } from 'react-window';
import Line from './line';

const initialInstructions = Array.from({length: 0x10000}, (_, i) => ({
  line: i,
  mnemonic: "NOP",
  operands: "nn,n"
}));

export interface Instruction {
  line: number;
  mnemonic: string;
  operands: string;
}

interface Props {
  instructions?: Instruction[];
}

export const MemoryMap: FunctionComponent<Props> = ({instructions}) => {
    const [lastInstructions, setLastInstructions] = useState<Instruction[]>(initialInstructions);

    useEffect(() => {
      if (instructions) {
        setLastInstructions(instructions);
      }
    }, [instructions]);

    return (
      <List
        className="memory-map"
        itemSize={20}
        height={500}
        width={150}
        itemCount={0x10000}
        itemData={lastInstructions}
      >
        {Line}
      </List>
    );
  };
  
  export default MemoryMap;