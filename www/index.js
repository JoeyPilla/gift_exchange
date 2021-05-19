import * as wasm from "gift-exchange";

wasm.greet(JSON.stringify({ name: "joey" }));

let g = wasm.gift_exchange(
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
);

document.getElementById("graph").innerText = g;
