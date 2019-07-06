import React, { FunctionComponent } from 'react';
import { Gameboy as GameboyType } from 'rustyboy-web';

import Gameboy from './gameboy';
import Debugger from './debugger';

interface Props {
  gameboy: GameboyType
}

export const Emulator: FunctionComponent<Props> = ({ gameboy }) => {
  return (
    <div>
      <Gameboy gameboy={gameboy} />
      <Debugger loaded={false} />
    </div>
  );
};

export default Emulator;