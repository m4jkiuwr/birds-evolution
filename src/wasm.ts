import init, * as wasm from "../libs/simulation-wasm/pkg"


export const wasmReady = init()
export const { Simulation, World, Animal, Food, test_fun } = wasm 
