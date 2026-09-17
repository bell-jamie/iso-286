/**
 * Which side of a fit a tolerance applies to.
 * @enum {0 | 1}
 */
export const Feature = Object.freeze({
    /**
     * An internal feature, taking an upper case class such as `"H7"`.
     */
    Hole: 0, "0": "Hole",
    /**
     * An external feature, taking a lower case class such as `"h7"`.
     */
    Shaft: 1, "1": "Shaft",
});

/**
 * One tolerance class, and the nominal size it was matched at.
 *
 * The deviations are held flat rather than as a [`Tolerance`], so that reading one
 * across the wasm boundary is a plain number rather than a further handle. Use
 * [`Match::tolerance`] to get them back as a [`Tolerance`].
 */
export class Match {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Match.prototype);
        obj.__wbg_ptr = ptr;
        MatchFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MatchFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_match_free(ptr, 0);
    }
    /**
     * Tolerance class, e.g. `"H7"`.
     * @returns {string}
     */
    get class() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_match_class(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Distance from the wanted tolerance in millimetres: the differences in upper
     * limit, lower limit and midpoint, added together. Zero is an exact match.
     * @returns {number}
     */
    get error() {
        const ret = wasm.__wbg_get_match_error(this.__wbg_ptr);
        return ret;
    }
    /**
     * Lower deviation in millimetres, as `limits` would report it.
     * @returns {number}
     */
    get lower() {
        const ret = wasm.__wbg_get_match_lower(this.__wbg_ptr);
        return ret;
    }
    /**
     * Midpoint of the two deviations in millimetres.
     * @returns {number}
     */
    get middle() {
        const ret = wasm.__wbg_get_match_middle(this.__wbg_ptr);
        return ret;
    }
    /**
     * Nominal size in millimetres. Equal to the size supplied under a strict search.
     * @returns {number}
     */
    get size() {
        const ret = wasm.__wbg_get_match_size(this.__wbg_ptr);
        return ret;
    }
    /**
     * Upper deviation in millimetres, as `limits` would report it.
     * @returns {number}
     */
    get upper() {
        const ret = wasm.__wbg_get_match_upper(this.__wbg_ptr);
        return ret;
    }
    /**
     * Tolerance class, e.g. `"H7"`.
     * @param {string} arg0
     */
    set class(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_match_class(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Distance from the wanted tolerance in millimetres: the differences in upper
     * limit, lower limit and midpoint, added together. Zero is an exact match.
     * @param {number} arg0
     */
    set error(arg0) {
        wasm.__wbg_set_match_error(this.__wbg_ptr, arg0);
    }
    /**
     * Lower deviation in millimetres, as `limits` would report it.
     * @param {number} arg0
     */
    set lower(arg0) {
        wasm.__wbg_set_match_lower(this.__wbg_ptr, arg0);
    }
    /**
     * Midpoint of the two deviations in millimetres.
     * @param {number} arg0
     */
    set middle(arg0) {
        wasm.__wbg_set_match_middle(this.__wbg_ptr, arg0);
    }
    /**
     * Nominal size in millimetres. Equal to the size supplied under a strict search.
     * @param {number} arg0
     */
    set size(arg0) {
        wasm.__wbg_set_match_size(this.__wbg_ptr, arg0);
    }
    /**
     * Upper deviation in millimetres, as `limits` would report it.
     * @param {number} arg0
     */
    set upper(arg0) {
        wasm.__wbg_set_match_upper(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Match.prototype[Symbol.dispose] = Match.prototype.free;

export class Tolerance {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Tolerance.prototype);
        obj.__wbg_ptr = ptr;
        ToleranceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ToleranceFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tolerance_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get lower() {
        const ret = wasm.__wbg_get_tolerance_lower(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get middle() {
        const ret = wasm.__wbg_get_tolerance_middle(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get upper() {
        const ret = wasm.__wbg_get_tolerance_upper(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set lower(arg0) {
        wasm.__wbg_set_tolerance_lower(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set middle(arg0) {
        wasm.__wbg_set_tolerance_middle(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set upper(arg0) {
        wasm.__wbg_set_tolerance_upper(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Tolerance.prototype[Symbol.dispose] = Tolerance.prototype.free;

/**
 * @param {number} size
 * @param {string} tolerance_class
 * @returns {Match}
 */
export function findPreferred(size, tolerance_class) {
    const ptr0 = passStringToWasm0(tolerance_class, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.findPreferred(size, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return Match.__wrap(ret[0]);
}

/**
 * @returns {string[]}
 */
export function grades() {
    const ret = wasm.grades();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @returns {string[]}
 */
export function holeDeviations() {
    const ret = wasm.holeDeviations();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @returns {string[]}
 */
export function holePreferredTolerances() {
    const ret = wasm.holePreferredTolerances();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @param {number} size
 * @param {string} tolerance_class
 * @returns {Tolerance}
 */
export function limits(size, tolerance_class) {
    const ptr0 = passStringToWasm0(tolerance_class, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.limits(size, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return Tolerance.__wrap(ret[0]);
}

/**
 * `results` and `decimals` are signed because JavaScript has no unsigned integer to
 * pass: a negative number reaches an unsigned parameter as a very large positive one,
 * silently, so it has to be caught on this side.
 * @param {number} size
 * @param {number} upper
 * @param {number} lower
 * @param {Feature} feature
 * @param {boolean} strict
 * @param {number} results
 * @param {number} decimals
 * @returns {Match[]}
 */
export function listClosest(size, upper, lower, feature, strict, results, decimals) {
    const ret = wasm.listClosest(size, upper, lower, feature, strict, results, decimals);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @param {number} size
 * @param {string} tolerance_class
 * @returns {Match[]}
 */
export function listPreferred(size, tolerance_class) {
    const ptr0 = passStringToWasm0(tolerance_class, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.listPreferred(size, ptr0, len0);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @returns {string[]}
 */
export function shaftDeviations() {
    const ret = wasm.shaftDeviations();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

/**
 * @returns {string[]}
 */
export function shaftPreferredTolerances() {
    const ret = wasm.shaftPreferredTolerances();
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}
export function __wbg_Error_83742b46f01ce22d(arg0, arg1) {
    const ret = Error(getStringFromWasm0(arg0, arg1));
    return ret;
}
export function __wbg___wbindgen_throw_6ddd609b62940d55(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_match_new(arg0) {
    const ret = Match.__wrap(arg0);
    return ret;
}
export function __wbindgen_cast_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
const MatchFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_match_free(ptr >>> 0, 1));
const ToleranceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tolerance_free(ptr >>> 0, 1));

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
