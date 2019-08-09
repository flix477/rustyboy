import React, {FunctionComponent} from 'react';
import {ListChildComponentProps} from 'react-window';
import './line.css';

export interface Instruction {
  line: number;
  mnemonic: string;
  operands: string;
}

function formatLine(line: number) {
  return line.toString(16).toUpperCase().padStart(4, '0');
}

function getClassName(line: number, currentLine: number): string {
  if (line === currentLine) {
    return 'line current-line';
  }

  return 'line';
}

const Line: FunctionComponent<ListChildComponentProps> = ({index, style, data}) => {
  const instruction = data.instructions[index] as Instruction;
  const formattedLine = formatLine(instruction.line);
  const className = getClassName(instruction.line, data.currentLine);
  const {mnemonic, operands} = instruction;

  return (
    <div className={className} style={style}>
      <span className="line-number">{formattedLine}</span>
      <div>
        <span className="line-mnemonic">{mnemonic}</span>
        <span className="line-operand">{operands}</span>
      </div>
    </div>
  );
};

export default Line;