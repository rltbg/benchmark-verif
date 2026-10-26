// EXPÉRIMENTAL ET NON TESTÉ.
// Le backend `--codegen FSharp` de F* n'est plus accompagné d'un runtime (le tarball F* 2025.03.25
// ne contient plus de ulib/fs ; seul ulib/ml existe). Ce fichier est un *squelette* du strict
// minimum utilisé par nos six modules : entiers machine non signés, conversions, et buffers
// natifs (tableaux .NET). Les noms de modules/fonctions suivent ce que le backend F# produit
// habituellement (FStar.UInt32.add_mod, LowStar.Buffer.index, ...) ; à ajuster selon la sortie
// réelle de `fstar.exe --codegen FSharp`.
namespace FStar

module Prims =
    type unit = Microsoft.FSharp.Core.unit

module UInt8 =
    type t = byte
    let inline v (x: t) : bigint = bigint (int x)
    let inline uint_to_t (x: bigint) : t = byte x
    let inline eq (a: t) (b: t) = a = b
    let inline sub_mod (a: t) (b: t) : t = a - b

module UInt32 =
    type t = uint32
    let inline v (x: t) : bigint = bigint x
    let inline uint_to_t (x: bigint) : t = uint32 x
    let inline add (a: t) (b: t) : t = a + b
    let inline add_mod (a: t) (b: t) : t = a + b
    let inline sub (a: t) (b: t) : t = a - b
    let inline mul (a: t) (b: t) : t = a * b
    let inline mul_mod (a: t) (b: t) : t = a * b
    let inline div (a: t) (b: t) : t = a / b
    let inline rem (a: t) (b: t) : t = a % b
    let inline logand (a: t) (b: t) : t = a &&& b
    let inline lt (a: t) (b: t) = a < b
    let inline lte (a: t) (b: t) = a <= b
    let inline gt (a: t) (b: t) = a > b
    let inline gte (a: t) (b: t) = a >= b

module UInt64 =
    type t = uint64
    let inline v (x: t) : bigint = bigint x
    let inline uint_to_t (x: bigint) : t = uint64 x
    let inline add_mod (a: t) (b: t) : t = a + b
    let inline mul_mod (a: t) (b: t) : t = a * b
    let inline rem (a: t) (b: t) : t = a % b
    let inline logand (a: t) (b: t) : t = a &&& b
    let inline logxor (a: t) (b: t) : t = a ^^^ b
    let inline logor (a: t) (b: t) : t = a ||| b
    let inline shift_right (a: t) (s: UInt32.t) : t = a >>> int s
    let inline eq (a: t) (b: t) = a = b

module Int =
    module Cast =
        let inline uint32_to_uint64 (x: uint32) : uint64 = uint64 x
        let inline uint64_to_uint32 (x: uint64) : uint32 = uint32 x
        let inline uint32_to_uint8 (x: uint32) : byte = byte x
        let inline uint8_to_uint32 (x: byte) : uint32 = uint32 x

module HyperStack =
    let root = ()

namespace LowStar

module Buffer =
    type buffer<'a> = 'a[]
    let inline malloc (_r: unit) (init: 'a) (len: uint32) : 'a[] = Array.create (int len) init
    let inline free (_b: 'a[]) : unit = ()
    let inline index (b: 'a[]) (i: uint32) : 'a = b.[int i]
    let inline upd (b: 'a[]) (i: uint32) (v: 'a) : unit = b.[int i] <- v
