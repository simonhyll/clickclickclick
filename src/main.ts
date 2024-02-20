import { invoke } from "@tauri-apps/api/core";

//const toggle = document.querySelector('.toggle') as HTMLDivElement;
const input = document.getElementById('input') as HTMLInputElement;
//const off = document.querySelector('.off') as HTMLDivElement;
let delay = 1000;

invoke('set_delay', { delay: delay})

input?.addEventListener('input', (e) => {
    delay = parseInt((e.target as HTMLInputElement).value);
    if (isNaN(delay)) delay = 1000;
    if (delay < 1) delay = 1;
    console.log(delay)
});