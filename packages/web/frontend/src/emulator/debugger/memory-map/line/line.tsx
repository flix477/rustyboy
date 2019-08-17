import React, {FunctionComponent} from 'react';
import {ListChildComponentProps} from 'react-window';
import './line.css';

export interface Instruction {
  line: number;
  mnemonic: string;
  operands: string;
  isBreakpoint: boolean;
  onBreakpoint?: (line: number) => void;
}

function formatLine(line: number) {
  return line.toString(16).toUpperCase().padStart(4, '0');
}

function getClassName(line: number, currentLine: number, isBreakpoint: boolean): string {
  const className = ['line'];

  if (line === currentLine)
    className.push('current-line');

  if (isBreakpoint)
    className.push('breakpoint');

  return className.join(' ');
}

const Line: FunctionComponent<ListChildComponentProps> = ({index, style, data}) => {
  const instruction = data.instructions[index] as Instruction;
  const formattedLine = formatLine(instruction.line);
  const {mnemonic, operands, isBreakpoint, line} = instruction;
  const className = getClassName(line, data.currentLine, isBreakpoint);
  const onBreakpoint = () => {
    if (instruction.onBreakpoint)
      instruction.onBreakpoint(line);
  };

  return (
    <div className={className} style={style}>
      <div className="marker" />
      <span className="line-number" onClick={onBreakpoint}>{formattedLine}</span>
      <div>
        <span className="line-mnemonic">{mnemonic}</span>
        <span className="line-operand">{operands}</span>
      </div>
    </div>
  );
};

export default Line;