import init, * as wasm from "../libs/simulation-wasm/pkg"


export const wasmReady = init()
export const { Simulation, test_fun } = wasm 
