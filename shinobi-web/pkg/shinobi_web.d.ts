/* tslint:disable */
/* eslint-disable */
/**
 * @param {number} left
 * @param {number} right
 * @returns {number}
 */
export function add(left: number, right: number): number;
/**
 * @param {number} left
 * @param {number} right
 * @returns {number}
 */
export function multiply(left: number, right: number): number;
export class ClientEngine {
  free(): void;
  constructor();
  /**
   * @param {string} fen
   */
  load_fen(fen: string): void;
  /**
   * @returns {Array<any>}
   */
  recieve_position(): Array<any>;
  reset_position(): void;
  /**
   * @returns {(Move)[]}
   */
  moves(): (Move)[];
  /**
   * @param {Move} mv
   */
  make_move(mv: Move): void;
  /**
   * @param {number} depth
   * @returns {bigint}
   */
  start_perft(depth: number): bigint;
  /**
   * @returns {Move | undefined}
   */
  search(): Move | undefined;
}
export class Move {
  free(): void;
  0: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_clientengine_free: (a: number, b: number) => void;
  readonly clientengine_new: () => number;
  readonly clientengine_load_fen: (a: number, b: number, c: number, d: number) => void;
  readonly clientengine_recieve_position: (a: number) => number;
  readonly clientengine_reset_position: (a: number) => void;
  readonly clientengine_moves: (a: number, b: number) => void;
  readonly clientengine_make_move: (a: number, b: number) => void;
  readonly clientengine_start_perft: (a: number, b: number) => number;
  readonly clientengine_search: (a: number) => number;
  readonly add: (a: number, b: number) => number;
  readonly multiply: (a: number, b: number) => number;
  readonly __wbg_move_free: (a: number, b: number) => void;
  readonly __wbg_get_move_0: (a: number) => number;
  readonly __wbg_set_move_0: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
