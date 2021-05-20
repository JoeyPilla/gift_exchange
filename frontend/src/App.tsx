import React from "react";
import logo from "./logo.svg";
import "./App.css";

function App() {
  const [g, setG] = React.useState("");

  React.useEffect(() => {
    loadWasm();
  }, []);

  const loadWasm = async () => {
    try {
      const wasm = await import("gift-exchange");
      setG(
        wasm.gift_exchange(
          JSON.stringify({
            users: [
              { name: "XUqmv", dont: ["kouzk"] },
              { name: "kouzk", dont: ["kouzk"] },
              { name: "yShkw", dont: ["XUqmv", "XUqmv", "D7OUP"] },
              { name: "D7OUP", dont: ["kouzk"] },
              { name: "EA6yl", dont: [] },
              { name: "jcho4", dont: ["9ccLF"] },
              { name: "tMrKy", dont: ["xJmMe"] },
              { name: "HHOPK", dont: ["xJmMe", "xJmMe", "HHOPK"] },
              { name: "9ccLF", dont: ["xJmMe"] },
              { name: "xJmMe", dont: ["tMrKy"] },
            ],
          })
        )
      );
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  return (
    <div className="App">
      <h1>{g}</h1>
    </div>
  );
}

export default App;
