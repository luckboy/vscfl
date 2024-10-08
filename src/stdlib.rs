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
const STD_OPTION_SOURCE: &'static str = include_str!("stdlib/std_option.vscfl");

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
    Source::String(String::from("(stdlib)/std_impls.vscfl"), src)
}

pub fn stdlib_sources() -> Vec<Source>
{ 
    vec![
        Source::String(String::from("(stdlib)/lang.vscfl"), String::from(LANG_SOURCE)),
        generate_lang_impls_source(),
        Source::String(String::from("(stdlib)/std.vscfl"), String::from(STD_SOURCE)),
        Source::String(String::from("(stdlib)/std_option.vscfl"), String::from(STD_OPTION_SOURCE)),
        generate_std_impls_source()
    ]
}
