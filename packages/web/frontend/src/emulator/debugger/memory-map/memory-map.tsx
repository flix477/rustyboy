import React, { FunctionComponent, useState, useEffect } from 'react';
import { FixedSizeList as List } from 'react-window';
import Line from './line';

const emptyBus = new Uint8Array(0x10000);

interface Props {
  bus?: Uint8Array
}

export const MemoryMap: FunctionComponent<Props> = ({bus}) => {
    const [lastBus, setLastBus] = useState<Uint8Array>(emptyBus);

    useEffect(() => {
      if (bus) {
        setLastBus(bus);
      }
    }, [bus, setLastBus]);

    return (
      <List
        className="memory-map"
        itemSize={20}
        height={500}
        width={150}
        itemCount={0x10000}
        itemData={lastBus}
      >
        {Line}
      </List>
    );
  };
  
  export default MemoryMap;