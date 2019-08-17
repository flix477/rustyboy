import React, {FunctionComponent, useState, useEffect} from 'react';
import {Gameboy as GameboyType} from 'rustyboy-web';
import {imports} from './imports';

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
  const [hasDebugger, setHasDebugger] = useState(false);
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

  if (loading) return <p>Loading...</p>;

  return (
    <div className="container">
      {!gameboy && (
        <div className="fileSelectionContainer">
          <h1>Rustyboy</h1>
          <div className="cartridgeInput">
            <label htmlFor="cartridge">Load game</label>
            <input hidden id="cartridge" name="cartridge" type="file" accept=".gb" onChange={value => {
              if (value.target.files && value.target.files[0]) {
                setGame(value.target.files[0]);
              }
            }} />
          </div>
          <label htmlFor="hasDebugger" className="hasDebugger">
            <input
              id="hasDebugger"
              name="hasDebugger"
              type="checkbox"
              onChange={(e) => setHasDebugger(e.target.checked)}
            />
            <span>Debug mode</span>
          </label>
        </div>
      )}
      {gameboy && Emulator && (
        <Emulator hasDebugger={hasDebugger} gameboy={gameboy} />
      )}
    </div>
  );
};

export default App;
