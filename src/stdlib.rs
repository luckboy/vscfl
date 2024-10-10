//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
pub use crate::frontend::source::Source;

const LANG_SOURCE: &'static str = include_str!("stdlib/lang.vscfl");

fn generate_lang_impls_source() -> Source
{
    let mut src = String::new();
    // OpNeg
    for s in ["Char", "Short", "Int", "Long", "Half", "Float", "Double", "PtrdiffT", "IntptrT"] {
        src += format!("builtin impl OpNeg for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpNeg for {}{};\n", s, n).as_str();
        }
    }
    // OpNot
    for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpNot for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpNot for {}{};\n", s, n).as_str();
        }
    }
    // OpMul
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpMul for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpMul for {}{};\n", s, n).as_str();
        }
    }
    // OpDiv
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpDiv for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpDiv for {}{}\n;", s, n).as_str();
        }
    }
    // OpRem
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpRem for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpRem for {}{};\n", s, n).as_str();
        }
    }
    // OpAdd
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpAdd for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpAdd for {}{};\n", s, n).as_str();
        }
    }
    // OpSub
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpSub for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpSub for {}{};\n", s, n).as_str();
        }
    }
    // OpShl
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpShl for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpShl for {}{};\n", s, n).as_str();
        }
    }
    // OpShr
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpShr for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpShr for {}{};\n", s, n).as_str();
        }
    }
    // Eq
    for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl Eq for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Eq for {}{};\n", s, n).as_str();
        }
    }
    // Ord
    for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl Ord for {};\n", s).as_str();
    }
    // OpAnd
    for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpAnd for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpAnd for {}{};\n", s, n).as_str();
        }
    }
    // OpXor
    for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpXor for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpXor for {}{};\n", s, n).as_str();
        }
    }
    // OpOr
    for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl OpOr for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl OpOr for {}{};\n", s, n).as_str();
        }
    }
    // OpGet
    for s in ["Ref", "PrivateRef", "LocalRef", "GlobalRef", "ConstantRef", "UniqRef", "UniqPrivateRef", "UniqLocalRef", "UniqGlobalRef"] {
        src += format!("builtin impl OpGet for {};\n", s).as_str();
    }
    // OpSet
    for s in ["UniqRef", "UniqPrivateRef", "UniqLocalRef", "UniqGlobalRef"] {
        src += format!("builtin impl OpSet for {};\n", s).as_str();
    }
    // OpUpdate
    for s in ["UniqRef", "UniqPrivateRef", "UniqLocalRef", "UniqGlobalRef"] {
        src += format!("builtin impl OpUpdate for {};\n", s).as_str();
    }
    // OpGetNth
    src += "builtin impl OpGetNth for [_; _];\n";
    for s in ["Slice", "PrivateSlice", "LocalSlice", "GlobalSlice", "ConstantSlice", "UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
        src += format!("builtin impl OpGetNth for {};\n", s).as_str();
    }
    // OpSetNth
    src += "builtin impl OpSetNth for [_; _];\n";
    for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
        src += format!("builtin impl OpSetNth for {};\n", s).as_str();
    }
    // OpUpdateNth
    src += "builtin impl OpUpdateNth for [_; _];\n";
    for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
        src += format!("builtin impl OpUpdateNth for {};\n", s).as_str();
    }
    // SliceFrom
    src += "builtin impl SliceFrom for [_; _];\n";
    // PrivateSliceFrom
    src += "builtin impl PrivateSliceFrom for [_; _];\n";
    // LocalSliceFrom
    src += "builtin impl LocalSliceFrom for [_; _];\n";
    // GlobalSliceFrom
    src += "builtin impl GlobalSliceFrom for [_; _];\n";
    Source::String(String::from("(stdlib)/lang_impls.vscfl"), src)
}

const STD_SOURCE: &'static str = include_str!("stdlib/std.vscfl");
const STD_COMMON_SOURCE: &'static str = include_str!("stdlib/std_common.vscfl");
const STD_GEOMETRIC_SOURCE: &'static str = include_str!("stdlib/std_geometric.vscfl");
const STD_MATH_SOURCE: &'static str = include_str!("stdlib/std_math.vscfl");
const STD_OPTION_SOURCE: &'static str = include_str!("stdlib/std_option.vscfl");
const STD_RANGE_SOURCE: &'static str = include_str!("stdlib/std_range.vscfl");
const STD_VALUES_SOURCE: &'static str = include_str!("stdlib/std_values.vscfl");

fn generate_std_impls_source() -> Source
{
    let mut src = String::new();
    // Zero
    for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
        src += format!("builtin impl Zero for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Zero for {}{};\n", s, n).as_str();
        }
    }
    // ShlN
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Shl{} for {}{};\n", n, s, n).as_str();
        }
    }
    // ShrN
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Shr{} for {}{};\n", n, s, n).as_str();
        }
    }
    // Len
    src += "builtin impl Len for [_; _];\n";
    for s in ["Slice", "PrivateSlice", "LocalSlice", "GlobalSlice", "ConstantSlice", "UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
        src += format!("builtin impl Len for {};\n", s).as_str();
    }
    // GetRef
    src += "builtin impl GetRef for Slice;\n";
    // GetPrivateRef
    src += "builtin impl GetPrivateRef for PrivateSlice;\n";
    // GetLocalRef
    src += "builtin impl GetLocalRef for LocalSlice;\n";
    // GetGlobalRef
    src += "builtin impl GetGlobalRef for GlobalSlice;\n";
    // GetConstantRef
    src += "builtin impl GetConstantRef for ConstantSlice;\n";
    // UpdateUniqRef
    src += "builtin impl UpdateUniqRef for UniqSlice;\n";
    // UpdateUniqPrivateRef
    src += "builtin impl UpdateUniqPrivateRef for UniqPrivateSlice;\n";
    // UpdateUniqLocalRef
    src += "builtin impl UpdateUniqLocalRef for UniqLocalSlice;\n";
    // UpdateUniqGlobalRef
    src += "builtin impl UpdateUniqGlobalRef for UniqGlobalSlice;\n";
    // GetSlice
    src += "builtin impl GetSlice for Slice;\n";
    // GetPrivateSlice
    src += "builtin impl GetPrivateSlice for PrivateSlice;\n";
    // GetLocalSlice
    src += "builtin impl GetLocalSlice for LocalSlice;\n";
    // GetGlobalSlice
    src += "builtin impl GetGlobalSlice for GlobalSlice;\n";
    // GetConstantSlice
    src += "builtin impl GetConstantSlice for ConstantSlice;\n";
    // UpdateUniqSlice
    src += "builtin impl UpdateUniqSlice for UniqSlice;\n";
    // UpdateUniqPrivateSlice
    src += "builtin impl UpdateUniqPrivateSlice for UniqPrivateSlice;\n";
    // UpdateUniqLocalSlice
    src += "builtin impl UpdateUniqLocalSlice for UniqLocalSlice;\n";
    // UpdateUniqGlobalSlice
    src += "builtin impl UpdateUniqGlobalSlice for UniqGlobalSlice;\n";
    // Map
    src += "builtin impl Map for [_; _];\n";
    // MapInPlace
    src += "builtin impl MapInPlace for [_; _];\n";
    for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
        src += format!("builtin impl MapInPlace for {};\n", s).as_str();
    }
    // Fold
    src += "builtin impl Fold for [_; _];\n";
    for s in ["Slice", "PrivateSlice", "LocalSlice", "GlobalSlice", "ConstantSlice", "UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
        src += format!("builtin impl Fold for {};\n", s).as_str();
    }
    // FoldUpdate
    src += "builtin impl FoldUpdate for [_; _];\n";
    for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
        src += format!("builtin impl FoldUpdate for {};\n", s).as_str();
    }
    // Zip
    src += "builtin impl Zip for [_; _];\n";
    // Unzip
    src += "builtin impl Unzip for [_; _];\n";
    // MapInPlaceUniqRefs
    src += "builtin impl MapInPlaceUniqRefs for UniqSlice;\n";
    // MapInPlaceUniqPrivateRefs
    src += "builtin impl MapInPlaceUniqPrivateRefs for UniqPrivateSlice;\n";
    // MapInPlaceUniqLocalRefs
    src += "builtin impl MapInPlaceUniqLocalRefs for UniqLocalSlice;\n";
    // MapInPlaceUniqGlobalRefs
    src += "builtin impl MapInPlaceUniqGlobalRefs for UniqGlobalSlice;\n";
    // FoldUpdateUniqRefs
    src += "builtin impl FoldUpdateUniqRefs for UniqSlice;\n";
    // FoldUpdateUniqPrivateRefs
    src += "builtin impl FoldUpdateUniqPrivateRefs for UniqPrivateSlice;\n";
    // FoldUpdateUniqLocalRefs
    src += "builtin impl FoldUpdateUniqLocalRefs for UniqLocalSlice;\n";
    // FoldUpdateUniqGlobalRefs
    src += "builtin impl FoldUpdateUniqGlobalRefs for UniqGlobalSlice;\n";
    // Trigonometric
    for s in ["Half", "Float", "Double"] {
        src += format!("builtin impl Trigonometric for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Trigonometric for {}{};\n", s, n).as_str();
        }
    }
    // TrigonometricExt
    for s in ["Float", "Double"] {
        src += format!("builtin impl TrigonometricExt for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl TrigonometricExt for {}{};\n", s, n).as_str();
        }
    }
    // InvTrigonometric
    for s in ["Float", "Double"] {
        src += format!("builtin impl InvTrigonometric for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl InvTrigonometric for {}{};\n", s, n).as_str();
        }
    }
    // InvTrigonometric
    for s in ["Float", "Double"] {
        src += format!("builtin impl InvTrigonometricExt for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl InvTrigonometricExt for {}{};\n", s, n).as_str();
        }
    }
    // Hyperbolic
    for s in ["Float", "Double"] {
        src += format!("builtin impl Hyperbolic for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Hyperbolic for {}{};\n", s, n).as_str();
        }
    }
    // InvHyperbolic
    for s in ["Float", "Double"] {
        src += format!("builtin impl InvHyperbolic for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl InvHyperbolic for {}{};\n", s, n).as_str();
        }
    }
    // Erf
    for s in ["Float", "Double"] {
        src += format!("builtin impl Erf for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Erf for {}{};\n", s, n).as_str();
        }
    }
    // Gamma
    for s in ["Float", "Double"] {
        src += format!("builtin impl Gamma for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Gamma for {}{};\n", s, n).as_str();
        }
    }
    // LgammaR
    for s in ["Float", "Double"] {
        src += format!("builtin impl LgammaR for {};\n", s).as_str();
    }
    // LgammaRN
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl LgammaR{} for {}{};\n", n, s, n).as_str();
        }
    }
    // Math
    for s in ["Half", "Float", "Double"] {
        src += format!("builtin impl Math for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Math for {}{};\n", s, n).as_str();
        }
    }
    // MathExt
    for s in ["Float", "Double"] {
        src += format!("builtin impl MathExt for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl MathExt for {}{};\n", s, n).as_str();
        }
    }
    // Frexp
    for s in ["Float", "Double"] {
        src += format!("builtin impl Frexp for {};\n", s).as_str();
    }
    // FrexpN
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Frexp{} for {}{};\n", n, s, n).as_str();
        }
    }
    // Ilogb
    for s in ["Float", "Double"] {
        src += format!("builtin impl Ilogb for {};\n", s).as_str();
    }
    // IlogbN
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Ilogb{} for {}{};\n", n, s, n).as_str();
        }
    }
    // Ldexp
    for s in ["Float", "Double"] {
        src += format!("builtin impl Ldexp for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Ldexp{} for {}{};\n", n, s, n).as_str();
        }
    }
    // NanUint
    src += "builtin impl NanUint for Float;\n";
    // NanUintN
    for n in [2, 3, 4, 8, 16] {
        src += format!("builtin impl NanUint{} for Float{};\n", n, n).as_str();
    }
    // NanUlong
    src += "builtin impl NanUlong for Double;\n";
    // NanUlongN
    for n in [2, 3, 4, 8, 16] {
        src += format!("builtin impl NanUlong{} for Double{};\n", n, n).as_str();
    }
    // Pown
    for s in ["Float", "Double"] {
        src += format!("builtin impl Pown for {};\n", s).as_str();
    }
    // PownN
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Pown{} for {}{};\n", n, s, n).as_str();
        }
    }
    // Remquo
    for s in ["Float", "Double"] {
        src += format!("builtin impl Remquo for {};\n", s).as_str();
    }
    // RemquoN
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Remquo{} for {}{};\n", n, s, n).as_str();
        }
    }
    // Rootn
    for s in ["Float", "Double"] {
        src += format!("builtin impl Rootn for {};\n", s).as_str();
    }
    // RootnN
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Rootn{} for {}{};\n", n, s, n).as_str();
        }
    }
    // Fpclassify
    for s in ["Float", "Double"] {
        src += format!("builtin impl Fpclassify for {};\n", s).as_str();
    }
    // Signbit
    for s in ["Float", "Double"] {
        src += format!("builtin impl Signbit for {};\n", s).as_str();
    }
    // MathValues
    for s in ["Float", "Double"] {
        src += format!("builtin impl MathValues for {};\n", s).as_str();
    }
    // EpsilonValue
    for s in ["Float", "Double"] {
        src += format!("builtin impl EpsilonValue for {};\n", s).as_str();
    }
    // Common
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        src += format!("builtin impl Common for {};\n", s).as_str();
    }
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl Common for {}{};\n", s, n).as_str();
        }
    }
    // CommonExt
    for s in ["Float", "Double"] {
        src += format!("builtin impl CommonExt for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("builtin impl CommonExt for {}{};\n", s, n).as_str();
        }
    }
    // MaxValue
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        src += format!("builtin impl MaxValue for {};\n", s).as_str();
    }
    // MinValue
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        src += format!("builtin impl MinValue for {};\n", s).as_str();
    }
    // Cross
    for s in ["Float", "Double"] {
        for n in [3, 4] {
            src += format!("builtin impl Cross for {}{};\n", s, n).as_str();
        }
    }
    // HalfGeometric
    src += "builtin impl HalfGeometric for Float;\n";
    for n in [2, 3, 4] {
        src += format!("builtin impl HalfGeometric for Float{};\n", n).as_str();
    }
    // FloatGeometric
    src += "builtin impl FloatGeometric for Float;\n";
    for n in [2, 3, 4] {
        src += format!("builtin impl FloatGeometric for Float{};\n", n).as_str();
    }
    // DoubleGeometric
    src += "builtin impl DoubleGeometric for Double;\n";
    for n in [2, 3, 4] {
        src += format!("builtin impl DoubleGeometric for Double{};\n", n).as_str();
    }
    // Normalize
    for s in ["Float", "Double"] {
        src += format!("builtin impl Normalize for {};\n", s).as_str();
    }
    for s in ["Float", "Double"] {
        for n in [2, 3, 4] {
            src += format!("builtin impl Normalize for {}{};\n", s, n).as_str();
        }
    }
    Source::String(String::from("(stdlib)/std_impls.vscfl"), src)
}

fn generate_opencl_convert_source() -> Source
{
    let mut src = String::new();
    // ConvertS
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        src += format!("trait Convert{}\n", s).as_str();
        src += "{\n";
        src += format!("    convert_{}(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_rte(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_rtz(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_rtp(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_rtn(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_sat(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_sat_rte(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_sat_rtz(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_sat_rtp(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += format!("    convert_{}_sat_rtn(x: t) -> {} where t: Convert{};\n", s.to_lowercase(), s, s).as_str();
        src += "};\n";
    }
    // ConvertSN
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for n in [2, 3, 4, 8, 16] {
            src += format!("trait Convert{}{}\n", s, n).as_str();
            src += "{\n";
            src += format!("    convert_{}{}(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_rte(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_rtz(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_rtp(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_rtn(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_sat(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_sat_rte(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_sat_rtz(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_sat_rtp(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += format!("    convert_{}{}_sat_rtn(x: t) -> {}{} where t: Convert{}{};\n", s.to_lowercase(), n, s, n, s, n).as_str();
            src += "};\n";
        }
    }
    Source::String(String::from("(stdlib)/opencl_convert.vscfl"), src)
}

fn generate_opencl_impls_source() -> Source
{
    let mut src = String::new();
    // ConvertS
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for t in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            src += format!("builtin impl Convert{} for {};\n", s, t).as_str();
        }
    }
    // ConvertSN
    for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
        for t in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                src += format!("builtin impl Convert{}{} for {}{};\n", s, n, t, n).as_str();
            }
        }
    }
    Source::String(String::from("(stdlib)/opencl_impls.vscfl"), src)
}

pub fn stdlib_sources() -> Vec<Source>
{ 
    vec![
        Source::String(String::from("(stdlib)/lang.vscfl"), String::from(LANG_SOURCE)),
        generate_lang_impls_source(),
        Source::String(String::from("(stdlib)/std.vscfl"), String::from(STD_SOURCE)),
        Source::String(String::from("(stdlib)/std_common.vscfl"), String::from(STD_COMMON_SOURCE)),
        Source::String(String::from("(stdlib)/std_geometric.vscfl"), String::from(STD_GEOMETRIC_SOURCE)),
        Source::String(String::from("(stdlib)/std_math.vscfl"), String::from(STD_MATH_SOURCE)),
        Source::String(String::from("(stdlib)/std_option.vscfl"), String::from(STD_OPTION_SOURCE)),
        Source::String(String::from("(stdlib)/std_range.vscfl"), String::from(STD_RANGE_SOURCE)),
        Source::String(String::from("(stdlib)/std_values.vscfl"), String::from(STD_VALUES_SOURCE)),
        generate_std_impls_source(),
        generate_opencl_convert_source(),
        generate_opencl_impls_source()
    ]
}
