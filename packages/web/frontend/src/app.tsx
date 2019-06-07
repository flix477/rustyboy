import React, {FunctionComponent, useState, useEffect} from 'react';

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

const App: FunctionComponent = () => {
  const wasm = useWasm();
  const [game, setGame] = useState<Blob>();
  const loading = !Boolean(wasm);

  useEffect(() => {
    async function load() {
      if (game && !loading) {
        try {
          const arrayBuffer = await new Response(game).arrayBuffer();
          const uint8View = new Uint8Array(arrayBuffer);
          wasm.run(uint8View);
          console.log("do the wasm");
        } catch (err) {
          console.error(err);
        }
      }
    }

    load();
  }, [game, loading, wasm]);

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
