//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { Player } from "./api.js";
let account = "1234";
let player = new Player(account, "http://localhost:3000");
async function main() {
  //let towerId = 10038n + y;
  await player.deposit(15000n);
  await player.place(3n);
  let state = await player.getState();
  console.log(state);
}

main();

