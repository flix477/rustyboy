import React, { FunctionComponent } from 'react';
import { FixedSizeList as List } from 'react-window';
import Line from './line';

export const MemoryMap: FunctionComponent = () => {
    return (
      <List
        className="memory-map"
        itemSize={20}
        height={500}
        width={150}
        itemCount={0x10000}
      >
        {Line}
      </List>
    );
  };
  
  export default MemoryMap;