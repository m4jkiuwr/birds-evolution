import init, * as wasm from "lib-simulation-wasm"

await init();
export const { Simulation, World, Animal, Food, Statistics } = wasm 
