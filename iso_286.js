/* @ts-self-types="./iso_286.d.ts" */

import * as wasm from "./iso_286_bg.wasm";
import { __wbg_set_wasm } from "./iso_286_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Feature, Match, Tolerance, findPreferred, grades, holeDeviations, holePreferredTolerances, limits, listClosest, listPreferred, shaftDeviations, shaftPreferredTolerances
} from "./iso_286_bg.js";
