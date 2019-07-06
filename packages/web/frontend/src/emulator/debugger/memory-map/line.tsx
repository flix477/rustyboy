import React, {FunctionComponent} from 'react';
import {ListChildComponentProps} from 'react-window';
import './line.css';

function formatLine(line: number) {
  return line.toString(16).toUpperCase().padStart(4, '0');
}

const Line: FunctionComponent<ListChildComponentProps> = ({index, style}) => {
  const formattedLine = formatLine(index);
  const mnemonic = "LD";
  const operand = "n,n"

  return (
    <div className="line" style={style}>
      <span className="line-number">{formattedLine}</span>
      <div>
        <span className="line-mnemonic">{mnemonic}</span>
        <span className="line-operand">{operand}</span>
      </div>
    </div>
  );
}

export default Line;