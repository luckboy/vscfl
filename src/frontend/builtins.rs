//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashMap;
use std::collections::HashSet;
use crate::frontend::tree::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum RefTypeFlag
{
    None,
    Ref,
    Slice,
}

#[derive(Clone, Debug)]
pub struct BuiltinTypeVar
{
    pub type_arg_source: String,
    pub field_type_sources: Vec<String>,
    pub field_indices: Vec<(String, usize)>,
    pub shared_flag: SharedFlag,
    pub ref_type_flag: RefTypeFlag,
    pub is_primitive: bool,
    pub is_printable: bool,
}

impl BuiltinTypeVar
{
    pub fn new(type_arg_src: String, field_type_srcs: Vec<String>, field_idxs: Vec<(String, usize)>, shared_flag: SharedFlag, ref_type_flag: RefTypeFlag, is_primitive: bool, is_printable: bool) -> Self
    {
        BuiltinTypeVar {
            type_arg_source: type_arg_src,
            field_type_sources: field_type_srcs,
            field_indices: field_idxs,
            shared_flag,
            ref_type_flag,
            is_primitive,
            is_printable,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BuiltinVar
{
    pub type_source: String,
    pub where_source: String,
}

impl BuiltinVar
{
    pub fn new(type_src: String, where_src: String) -> Self
    { BuiltinVar { type_source: type_src, where_source: where_src, } }
}

#[derive(Clone, Debug)]
pub struct Builtins
{
    type_vars: HashMap<String, BuiltinTypeVar>,
    vars: HashMap<String, BuiltinVar>,
    impl_pairs: HashSet<(String, TypeName)>,
    impl_var_tuples: HashSet<(String, TypeName, String)>,
}

impl Builtins
{
    pub fn new() -> Self
    {
        //
        // Type variables.
        //
        let mut type_vars: HashMap<String, BuiltinTypeVar> = HashMap::new();
        // Type variables for standard library.
        type_vars.insert(String::from("Bool"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, false, false));
        type_vars.insert(String::from("Char"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Short"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Int"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Long"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Uchar"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Ushort"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Uint"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Ulong"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Half"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, false, false));
        type_vars.insert(String::from("Float"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("Double"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, true));
        type_vars.insert(String::from("SizeT"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, false));
        type_vars.insert(String::from("PtrdiffT"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, false));
        type_vars.insert(String::from("IntptrT"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, false));
        type_vars.insert(String::from("UintptrT"), BuiltinTypeVar::new(String::new(), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::None, true, false));
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                let mut field_type_srcs: Vec<String> = Vec::new();
                for _ in 0..n {
                    field_type_srcs.push(String::from(s));
                }
                let mut field_idxs: Vec<(String, usize)> = Vec::new();
                if n <= 4 {
                    if n >= 1 {
                        field_idxs.push((String::from("x"), 0));
                    }
                    if n >= 2 {
                        field_idxs.push((String::from("y"), 1));
                    }
                    if n >= 3 {
                        field_idxs.push((String::from("z"), 2));
                    }
                    if n >= 4 {
                        field_idxs.push((String::from("w"), 3));
                    }
                }
                for i in 0..n {
                    field_idxs.push((format!("s{:x}", i), i));
                }
                if n > 10 {
                    for i in 10..n {
                        field_idxs.push((format!("s{:X}", i), i));
                    }
                }
                type_vars.insert(format!("{}{}", s, n), BuiltinTypeVar::new(String::new(), field_type_srcs, field_idxs, SharedFlag::Shared, RefTypeFlag::None, false, true));
            }
        }
        type_vars.insert(String::from("Ref"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("PrivateRef"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("LocalRef"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("GlobalRef"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("ConstantRef"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("UniqRef"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::None, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("UniqPrivateRef"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::None, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("UniqLocalRef"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::None, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("UniqGlobalRef"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::None, RefTypeFlag::Ref, false, false));
        type_vars.insert(String::from("Slice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Slice, false, true));
        type_vars.insert(String::from("PrivateSlice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Slice, false, true));
        type_vars.insert(String::from("LocalSlice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Slice, false, false));
        type_vars.insert(String::from("GlobalSlice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Slice, false, true));
        type_vars.insert(String::from("ConstantSlice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::Shared, RefTypeFlag::Slice, false, false));
        type_vars.insert(String::from("UniqSlice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::None, RefTypeFlag::Slice, false, true));
        type_vars.insert(String::from("UniqPrivateSlice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::None, RefTypeFlag::Slice, false, true));
        type_vars.insert(String::from("UniqLocalSlice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::None, RefTypeFlag::Slice, false, false));
        type_vars.insert(String::from("UniqGlobaSlice"), BuiltinTypeVar::new(String::from("t"), Vec::new(), Vec::new(), SharedFlag::None, RefTypeFlag::Slice, false, true));
        //
        // Variables.
        //
        // Variables for standard library.
        let mut vars: HashMap<String, BuiltinVar> = HashMap::new();
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                let mut type_src = String::new();
                type_src.push('(');
                let mut is_first = true;
                for _ in 0..n {
                    if is_first {
                        type_src.push_str(", ");
                    }
                    type_src.push_str(s);
                    is_first = false;
                }
                type_src.push_str(") -> ");
                type_src.push_str(format!("{}{}", s, n).as_str());
                vars.insert(format!("{}{}", s.to_lowercase(), n), BuiltinVar::new(type_src, String::new()));
            }
        }
        vars.insert(String::from("ref"), BuiltinVar::new(String::from("(t) -> Ref<t>"), String::new()));
        vars.insert(String::from("private_ref"), BuiltinVar::new(String::from("(t) -> PrivateRef<t>"), String::new()));
        vars.insert(String::from("local_ref"), BuiltinVar::new(String::from("(t) -> LocalRef<t>"), String::new()));
        vars.insert(String::from("global_ref"), BuiltinVar::new(String::from("(t) -> GlobalRef<t>"), String::new()));
        vars.insert(String::from("constant_ref"), BuiltinVar::new(String::from("(t) -> ConstantRef<t>"), String::new()));
        vars.insert(String::from("uniq_ref"), BuiltinVar::new(String::from("(t) -> UniqRef<t>"), String::new()));
        vars.insert(String::from("uniq_private_ref"), BuiltinVar::new(String::from("(t) -> UniqPrivateRef<t>"), String::new()));
        vars.insert(String::from("uniq_local_ref"), BuiltinVar::new(String::from("(t) -> UniqLocalRef<t>"), String::new()));
        vars.insert(String::from("uniq_global_ref"), BuiltinVar::new(String::from("(t) -> UniqGlobalRef<t>"), String::new()));
        //
        // Implementations.
        //
        let mut impl_pairs: HashSet<(String, TypeName)> = HashSet::new();
        // Implementations for standard library.
        // OpNeg
        for s in ["Char", "Short", "Int", "Long", "Half", "Float", "Double", "PtrdiffT", "IntptrT"] {
            impl_pairs.insert((String::from("OpNeg"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpNeg"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpNot
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpNot"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpNot"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpMul
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpMul"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpMul"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpDiv
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpDiv"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpDiv"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpRem
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpRem"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpRem"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpAdd
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpAdd"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpAdd"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpSub
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpSub"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpSub"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpShl
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpShl"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpShl"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpShr
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpShr"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpShr"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Eq
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("Eq"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Eq"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Ord
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Half", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("Ord"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Ord"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpAnd
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpAnd"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpAnd"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpXor
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpXor"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpXor"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpOr
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            impl_pairs.insert((String::from("OpOr"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("OpOr"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // OpGet
        for s in ["Ref", "PrivateRef", "LocalRef", "GlobalRef", "ConstantRef", "UniqRef", "UniqPrivateRef", "UniqLocalRef", "UniqGlobalRef"] {
            impl_pairs.insert((String::from("OpGet"), TypeName::Name(String::from(s))));
        }
        // OpSet
        for s in ["UniqRef", "UniqPrivateRef", "UniqLocalRef", "UniqGlobalRef"] {
            impl_pairs.insert((String::from("OpSet"), TypeName::Name(String::from(s))));
        }
        // OpUpdate
        for s in ["UniqRef", "UniqPrivateRef", "UniqLocalRef", "UniqGlobalRef"] {
            impl_pairs.insert((String::from("OpUpdate"), TypeName::Name(String::from(s))));
        }
        // OpGetNth
        impl_pairs.insert((String::from("OpGetNth"), TypeName::Array(None)));
        for s in ["Slice", "PrivateSlice", "LocalSlice", "GlobalSlice", "ConstantSlice", "UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
            impl_pairs.insert((String::from("OpGetNth"), TypeName::Name(String::from(s))));
        }
        // OpSetNth
        impl_pairs.insert((String::from("OpSetNth"), TypeName::Array(None)));
        for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
            impl_pairs.insert((String::from("OpSetNth"), TypeName::Name(String::from(s))));
        }
        // OpUpdateNth
        impl_pairs.insert((String::from("OpUpdateNth"), TypeName::Array(None)));
        for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
            impl_pairs.insert((String::from("OpUpdateNth"), TypeName::Name(String::from(s))));
        }
        // SliceFrom
        impl_pairs.insert((String::from("SliceFrom"), TypeName::Array(None)));
        // PrivateSliceFrom
        impl_pairs.insert((String::from("PrivateSliceFrom"), TypeName::Array(None)));
        // LocalSliceFrom
        impl_pairs.insert((String::from("LocalSliceFrom"), TypeName::Array(None)));
        // GlobalSliceFrom
        impl_pairs.insert((String::from("GlobalSliceFrom"), TypeName::Array(None)));
        // ConstantSliceFrom
        impl_pairs.insert((String::from("ConstantSliceFrom"), TypeName::Array(None)));
        // UniqSliceFrom
        impl_pairs.insert((String::from("UniqSliceFrom"), TypeName::Array(None)));
        // UniqPrivateSliceFrom
        impl_pairs.insert((String::from("UniqPrivateSliceFrom"), TypeName::Array(None)));
        // UniqLocalSliceFrom
        impl_pairs.insert((String::from("UniqLocalSliceFrom"), TypeName::Array(None)));
        // UniqGlobalSliceFrom
        impl_pairs.insert((String::from("UniqGlobalSliceFrom"), TypeName::Array(None)));
        // GetRef
        impl_pairs.insert((String::from("GetRef"), TypeName::Name(String::from("Slice"))));
        // GetPrivateRef
        impl_pairs.insert((String::from("GetPrivateRef"), TypeName::Name(String::from("PrivateSlice"))));
        // GetLocalRef
        impl_pairs.insert((String::from("GetLocalRef"), TypeName::Name(String::from("LocalSlice"))));
        // GetGlobalRef
        impl_pairs.insert((String::from("GetGlobalRef"), TypeName::Name(String::from("GlobalSlice"))));
        // GetConstantRef
        impl_pairs.insert((String::from("GetConstantRef"), TypeName::Name(String::from("ConstantSlice"))));
        // GetUniqRef
        impl_pairs.insert((String::from("GetUniqRef"), TypeName::Name(String::from("UniqSlice"))));
        // GetUniqPrivateRef
        impl_pairs.insert((String::from("GetUniqPrivateRef"), TypeName::Name(String::from("UniqPrivateSlice"))));
        // GetUniqLocalRef
        impl_pairs.insert((String::from("GetUniqLocalRef"), TypeName::Name(String::from("UniqLocalSlice"))));
        // GetUniqGlobalRef
        impl_pairs.insert((String::from("GetUniqGlobalRef"), TypeName::Name(String::from("UniqGlobalSlice"))));
        // UpdateUniqRef
        impl_pairs.insert((String::from("UpdateUniqRef"), TypeName::Name(String::from("UniqSlice"))));
        // UpdateUniqPrivateRef
        impl_pairs.insert((String::from("UpdateUniqPrivateRef"), TypeName::Name(String::from("UniqPrivateSlice"))));
        // UpdateUniqLocalRef
        impl_pairs.insert((String::from("UpdateUniqLocalRef"), TypeName::Name(String::from("UniqLocalSlice"))));
        // UpdateUniqGlobalRef
        impl_pairs.insert((String::from("UpdateUniqGlobalRef"), TypeName::Name(String::from("UniqGlobalSlice"))));
        // GetSub
        for s in ["Ref", "PrivateRef", "LocalRef", "GlobalRef", "ConstantRef", "UniqRef", "UniqPrivateRef", "UniqLocalRef", "UniqGlobalRef"] {
            impl_pairs.insert((String::from("GetSub"), TypeName::Name(String::from(s))));
        }
        // UpdateSub
        for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
            impl_pairs.insert((String::from("UpdateSub"), TypeName::Name(String::from(s))));
        }
        // Map
        impl_pairs.insert((String::from("Map"), TypeName::Array(None)));
        // FlatMap
        impl_pairs.insert((String::from("FlatMap"), TypeName::Array(None)));
        // MapInPlace
        impl_pairs.insert((String::from("MapInPlace"), TypeName::Array(None)));
        for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
            impl_pairs.insert((String::from("MapInPlace"), TypeName::Name(String::from(s))));
        }
        // Fold
        impl_pairs.insert((String::from("Fold"), TypeName::Array(None)));
        for s in ["Slice", "PrivateSlice", "LocalSlice", "GlobalSlice", "ConstantSlice", "UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
            impl_pairs.insert((String::from("Fold"), TypeName::Name(String::from(s))));
        }
        // FoldUpdate
        impl_pairs.insert((String::from("FoldUpdate"), TypeName::Array(None)));
        for s in ["UniqSlice", "UniqPrivateSlice", "UniqLocalSlice", "UniqGlobalSlice"] {
            impl_pairs.insert((String::from("FoldUpdate"), TypeName::Name(String::from(s))));
        }
        // Zip
        impl_pairs.insert((String::from("Zip"), TypeName::Array(None)));
        // Unzip
        impl_pairs.insert((String::from("Unzip"), TypeName::Array(None)));
        // MapInPlaceUniqRefs
        impl_pairs.insert((String::from("MapInPlaceUniqRefs"), TypeName::Name(String::from("UniqSlice"))));
        // MapInPlaceUniqPrivateRefs
        impl_pairs.insert((String::from("MapInPlaceUniqPrivateRefs"), TypeName::Name(String::from("UniqPrivateSlice"))));
        // MapInPlaceUniqLocalRefs
        impl_pairs.insert((String::from("MapInPlaceUniqLocalRefs"), TypeName::Name(String::from("UniqLocalSlice"))));
        // MapInPlaceUniqGlobalRefs
        impl_pairs.insert((String::from("MapInPlaceUniqGlobalRefs"), TypeName::Name(String::from("UniqGlobalSlice"))));
        // FoldUpdateUniqRefs
        impl_pairs.insert((String::from("FoldUpdateUniqRefs"), TypeName::Name(String::from("UniqSlice"))));
        // FoldUpdatePrivateRefs
        impl_pairs.insert((String::from("FoldUpdateUniqPrivateRefs"), TypeName::Name(String::from("UniqPrivateSlice"))));
        // FoldUpdateUniqLocalRefs
        impl_pairs.insert((String::from("FoldUpdateUniqLocalRefs"), TypeName::Name(String::from("UniqLocalSlice"))));
        // FoldUpdateUniqGlobalRefs
        impl_pairs.insert((String::from("FoldUpdateUniqGlobalRefs"), TypeName::Name(String::from("UniqGlobalSlice"))));
        Builtins {
            type_vars,
            vars,
            impl_pairs,
            impl_var_tuples: HashSet::new(),
        }
    }

    pub fn new_empty() -> Self
    {
        Builtins {
            type_vars: HashMap::new(),
            vars: HashMap::new(),
            impl_pairs: HashSet::new(),
            impl_var_tuples: HashSet::new(),
        }
    }

    pub fn type_vars(&self) -> &HashMap<String, BuiltinTypeVar>
    { &self.type_vars }

    pub fn type_var(&self, ident: &String) -> Option<&BuiltinTypeVar>
    { self.type_vars.get(ident) }

    pub fn add_type_var(&mut self, ident: String, type_var: BuiltinTypeVar)
    { self.type_vars.insert(ident, type_var); }

    pub fn remove_type_var(&mut self, ident: &String) -> bool
    { self.type_vars.remove(ident).is_some() }

    pub fn vars(&self) -> &HashMap<String, BuiltinVar>
    { &self.vars }

    pub fn var(&self, ident: &String) -> Option<&BuiltinVar>
    { self.vars.get(ident) }

    pub fn add_var(&mut self, ident: String, var: BuiltinVar)
    { self.vars.insert(ident, var); }

    pub fn remove_var(&mut self, ident: &String) -> bool
    { self.vars.remove(ident).is_some() }
    
    pub fn impl_pairs(&self) -> &HashSet<(String, TypeName)>
    { &self.impl_pairs }
    
    pub fn has_impl_pair(&self, impl_pair: &(String, TypeName)) -> bool
    { self.impl_pairs.contains(impl_pair) }

    pub fn add_impl_pair(&mut self, impl_pair: (String, TypeName))
    { self.impl_pairs.insert(impl_pair); }

    pub fn remove_impl_pair(&mut self, impl_pair: &(String, TypeName))
    { self.impl_pairs.remove(impl_pair); }

    pub fn impl_var_tuples(&self) -> &HashSet<(String, TypeName, String)>
    { &self.impl_var_tuples }

    pub fn has_impl_var_tuple(&self, impl_var_tuple: &(String, TypeName, String)) -> bool
    { self.impl_var_tuples.contains(impl_var_tuple) }

    pub fn add_impl_var_tuple(&mut self, impl_var_tuple: (String, TypeName, String))
    { self.impl_var_tuples.insert(impl_var_tuple); }

    pub fn remove_impl_var_tuple(&mut self, impl_var_tuple: &(String, TypeName, String))
    { self.impl_var_tuples.remove(impl_var_tuple); }
}
