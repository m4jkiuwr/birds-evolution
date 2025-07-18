/* tslint:disable */
/* eslint-disable */
export class Animal {
  private constructor();
  free(): void;
  x: number;
  y: number;
  rotation: number;
}
export class Food {
  private constructor();
  free(): void;
  x: number;
  y: number;
}
export class Simulation {
  free(): void;
  constructor(animal_num: number, food_num: number);
  world(): World;
  step(): string;
  train(): string;
}
export class World {
  private constructor();
  free(): void;
  animals: Animal[];
  foods: Food[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_simulation_free: (a: number, b: number) => void;
  readonly simulation_new: (a: number, b: number) => number;
  readonly simulation_world: (a: number) => number;
  readonly simulation_step: (a: number) => [number, number];
  readonly simulation_train: (a: number) => [number, number];
  readonly __wbg_animal_free: (a: number, b: number) => void;
  readonly __wbg_get_animal_x: (a: number) => number;
  readonly __wbg_set_animal_x: (a: number, b: number) => void;
  readonly __wbg_get_animal_y: (a: number) => number;
  readonly __wbg_set_animal_y: (a: number, b: number) => void;
  readonly __wbg_get_animal_rotation: (a: number) => number;
  readonly __wbg_set_animal_rotation: (a: number, b: number) => void;
  readonly __wbg_food_free: (a: number, b: number) => void;
  readonly __wbg_world_free: (a: number, b: number) => void;
  readonly __wbg_get_world_animals: (a: number) => [number, number];
  readonly __wbg_set_world_animals: (a: number, b: number, c: number) => void;
  readonly __wbg_get_world_foods: (a: number) => [number, number];
  readonly __wbg_set_world_foods: (a: number, b: number, c: number) => void;
  readonly __wbg_set_food_x: (a: number, b: number) => void;
  readonly __wbg_set_food_y: (a: number, b: number) => void;
  readonly __wbg_get_food_x: (a: number) => number;
  readonly __wbg_get_food_y: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
