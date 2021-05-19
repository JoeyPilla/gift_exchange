import * as wasm from "gift-exchange";

wasm.greet(JSON.stringify({ name: "joey" }));

let g = wasm.gift_exchange(
  JSON.stringify({ users: [{ name: "joey", dont: [] }] })
);

document.getElementById("graph").innerText = g;
