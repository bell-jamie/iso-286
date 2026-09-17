/* tslint:disable */
/* eslint-disable */

/**
 * Which side of a fit a tolerance applies to.
 */
export enum Feature {
    /**
     * An internal feature, taking an upper case class such as `"H7"`.
     */
    Hole = 0,
    /**
     * An external feature, taking a lower case class such as `"h7"`.
     */
    Shaft = 1,
}

/**
 * One tolerance class, and the nominal size it was matched at.
 *
 * The deviations are held flat rather than as a [`Tolerance`], so that reading one
 * across the wasm boundary is a plain number rather than a further handle. Use
 * [`Match::tolerance`] to get them back as a [`Tolerance`].
 */
export class Match {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Tolerance class, e.g. `"H7"`.
     */
    class: string;
    /**
     * Distance from the wanted tolerance in millimetres: the differences in upper
     * limit, lower limit and midpoint, added together. Zero is an exact match.
     */
    error: number;
    /**
     * Lower deviation in millimetres, as `limits` would report it.
     */
    lower: number;
    /**
     * Midpoint of the two deviations in millimetres.
     */
    middle: number;
    /**
     * Nominal size in millimetres. Equal to the size supplied under a strict search.
     */
    size: number;
    /**
     * Upper deviation in millimetres, as `limits` would report it.
     */
    upper: number;
}

export class Tolerance {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    lower: number;
    middle: number;
    upper: number;
}

export function findPreferred(size: number, tolerance_class: string): Match;

export function grades(): string[];

export function holeDeviations(): string[];

export function holePreferredTolerances(): string[];

export function limits(size: number, tolerance_class: string): Tolerance;

/**
 * `results` and `decimals` are signed because JavaScript has no unsigned integer to
 * pass: a negative number reaches an unsigned parameter as a very large positive one,
 * silently, so it has to be caught on this side.
 */
export function listClosest(size: number, upper: number, lower: number, feature: Feature, strict: boolean, results: number, decimals: number): Match[];

export function listPreferred(size: number, tolerance_class: string): Match[];

export function shaftDeviations(): string[];

export function shaftPreferredTolerances(): string[];
