import init, {add, say_hi, say_hi_and_alert, say_hi_to_console} from "./pkg/hello_wasm_ftn.js";
await init();

console.log(add(1, 3));

document.getElementById("result").innerHTML = say_hi("Marko");

say_hi_and_alert("Jovana");
say_hi_to_console("Dinu");
say_hi_to_console("3");
say_hi_to_console(3);