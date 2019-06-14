import React, {FunctionComponent, useState, useEffect} from 'react';
import {Gameboy as GameboyType} from 'rustyboy-web';
import {imports} from './imports';

function useWasm() {
  const [wasm, setWasm] = useState();

  useEffect(() => {
    async function loadWasm() {
      try {
        setWasm(await imports());
      } catch (err) {}
    }

    loadWasm();
  }, [setWasm]);

  return wasm;
}

const App: FunctionComponent = () => {
  const imports = useWasm();
  const rustyboy = imports && imports.rustyboy;
  const Gameboy = imports && imports.Gameboy && imports.Gameboy.default;
  const [game, setGame] = useState<Blob>();
  const [gameboy, setGameboy] = useState<GameboyType>();
  const loading = !Boolean(imports);

  useEffect(() => {
    async function load() {
      if (game && !loading) {
        try {
          const arrayBuffer = await new Response(game).arrayBuffer();
          const uint8View = new Uint8Array(arrayBuffer);
          setGameboy(rustyboy.setup(uint8View));
        } catch (err) {
          console.error(err);
        }
      }
    }

    load();
  }, [game, loading, rustyboy]);

  return (
    <div>
      {loading && <p>Loading...</p>}
      {gameboy && Gameboy && <Gameboy gameboy={gameboy} />}
      <input type="file" accept=".gb" onChange={value => {
        if (value.target.files && value.target.files[0]) {
          setGame(value.target.files[0])
        }
      }} />
    </div>
  );
}

export default App;
