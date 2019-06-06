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
  const loading = !Boolean(wasm);

  return (
    <div className="App">
      {loading ? (
        <p>Loading...</p>
      ) : (
        <button onClick={wasm.greet}>Clickaroo</button>
      )}
    </div>
  );
}

export default App;
