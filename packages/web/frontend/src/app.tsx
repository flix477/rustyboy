import React, {FunctionComponent, useState, useEffect} from 'react';
import {Gameboy} from 'rustyboy-web';

function useWasm() {
  const [wasm, setWasm] = useState();

  useEffect(() => {
    async function loadWasm() {
      try {
        const rustyboy = await import('rustyboy-web');
        setWasm(rustyboy);
      } catch (err) {}
    }

    loadWasm();
  }, [setWasm]);

  return wasm;
}

function update(gameboy: Gameboy) {
  gameboy.run_to_vblank();
  requestAnimationFrame(() => update(gameboy));
}

const App: FunctionComponent = () => {
  const rustyboy = useWasm();
  const [game, setGame] = useState<Blob>();
  const loading = !Boolean(rustyboy);

  useEffect(() => {
    async function load() {
      if (game && !loading) {
        try {
          const arrayBuffer = await new Response(game).arrayBuffer();
          const uint8View = new Uint8Array(arrayBuffer);
          const gameboy = rustyboy.setup(uint8View);
          update(gameboy);
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
      <canvas id="canvas" />
      <input type="file" accept=".gb" onChange={value => {
        if (value.target.files && value.target.files[0]) {
          setGame(value.target.files[0])
        }
      }} />
    </div>
  );
}

export default App;
