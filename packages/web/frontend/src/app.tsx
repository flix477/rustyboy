import React, {FunctionComponent, useState, useEffect} from 'react';
import {Gameboy as GameboyType} from 'rustyboy-web';
import {imports} from './imports';
import {ReactComponent as Cartridge} from './cartridge-o.svg';

function useWasm() {
  const [wasm, setWasm] = useState();

  useEffect(() => {
    async function loadWasm() {
      try {
        setWasm(await imports());
      } catch (err) {
        alert('Error loading WebAssembly module: ' + err);
      }
    }

    loadWasm();
  }, [setWasm]);

  return wasm;
}

const App: FunctionComponent = () => {
  const imports = useWasm();
  const rustyboy = imports && imports.rustyboy;
  const Emulator = imports && imports.Emulator && imports.Emulator.default;
  const [game, setGame] = useState<Blob>();
  const [gameboy, setGameboy] = useState<GameboyType>();
  const loading = !imports;

  useEffect(() => {
    async function load() {
      if (game && !loading) {
        try {
          const arrayBuffer = await new Response(game).arrayBuffer();
          const uint8View = new Uint8Array(arrayBuffer);
          setGameboy(rustyboy.setup(uint8View));
        } catch (err) {
          alert('Error loading Rustyboy: ' + err);
        }
      }
    }

    load();
  }, [game, loading, rustyboy]);

  return (
    <div className="container">
      {loading && <p>Loading...</p>}
      {!gameboy && (
        <div className="fileSelectionContainer">
          <h1>Rustyboy</h1>
          {/* <div className="cartridges">
            <Cartridge className="cartridge" />
            <Cartridge className="cartridge" />
            <Cartridge className="cartridge" />
            <Cartridge className="cartridge" />
            <Cartridge className="cartridge" />
          </div> */}
          <div className="cartridgeInput">
            <label htmlFor="cartridge">Load game</label>
            <input hidden id="cartridge" name="cartridge" type="file" accept=".gb" onChange={value => {
              if (value.target.files && value.target.files[0]) {
                setGame(value.target.files[0]);
              }
            }} />
          </div>
        </div>
      )}
      {gameboy && Emulator && <Emulator gameboy={gameboy} />}
    </div>
  );
};

export default App;
