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
        // Type variables for language.
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
        // Variables for language.
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
        // Variables for OpenCl.
        vars.insert(String::from("get_work_dim"), BuiltinVar::new(String::from("() -> Uint"), String::new()));
        vars.insert(String::from("get_global_size"), BuiltinVar::new(String::from("(Uint) -> SizeT"), String::new()));
        vars.insert(String::from("get_global_id"), BuiltinVar::new(String::from("(Uint) -> SizeT"), String::new()));
        vars.insert(String::from("get_local_size"), BuiltinVar::new(String::from("(Uint) -> SizeT"), String::new()));
        vars.insert(String::from("get_local_id"), BuiltinVar::new(String::from("(Uint) -> SizeT"), String::new()));
        vars.insert(String::from("get_num_groups"), BuiltinVar::new(String::from("(Uint) -> SizeT"), String::new()));
        vars.insert(String::from("get_group_id"), BuiltinVar::new(String::from("(Uint) -> SizeT"), String::new()));
        vars.insert(String::from("get_global_offset"), BuiltinVar::new(String::from("(Uint) -> SizeT"), String::new()));
        //
        // Implementations.
        //
        let mut impl_pairs: HashSet<(String, TypeName)> = HashSet::new();
        // Implementations for language.
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
        // Implementations for standard library.
        // ShlN
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("Shl{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // ShrN
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("Shr{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
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
        // UpdateUniqRef
        impl_pairs.insert((String::from("UpdateUniqRef"), TypeName::Name(String::from("UniqSlice"))));
        // UpdateUniqPrivateRef
        impl_pairs.insert((String::from("UpdateUniqPrivateRef"), TypeName::Name(String::from("UniqPrivateSlice"))));
        // UpdateUniqLocalRef
        impl_pairs.insert((String::from("UpdateUniqLocalRef"), TypeName::Name(String::from("UniqLocalSlice"))));
        // UpdateUniqGlobalRef
        impl_pairs.insert((String::from("UpdateUniqGlobalRef"), TypeName::Name(String::from("UniqGlobalSlice"))));
        // GetSlice
        impl_pairs.insert((String::from("GetSlice"), TypeName::Name(String::from("Slice"))));
        // GetPrivateSlice
        impl_pairs.insert((String::from("GetPrivateSlice"), TypeName::Name(String::from("PrivateSlice"))));
        // GetLocalSlice
        impl_pairs.insert((String::from("GetLocalSlice"), TypeName::Name(String::from("LocalSlice"))));
        // GetGlobalSlice
        impl_pairs.insert((String::from("GetGlobalSlice"), TypeName::Name(String::from("GlobalSlice"))));
        // GetConstantSlice
        impl_pairs.insert((String::from("GetConstantSlice"), TypeName::Name(String::from("ConstantSlice"))));
        // UpdateUniqSlice
        impl_pairs.insert((String::from("UpdateUniqSlice"), TypeName::Name(String::from("UniqSlice"))));
        // UpdateUniqPrivateSlice
        impl_pairs.insert((String::from("UpdateUniqPrivateSlice"), TypeName::Name(String::from("UniqPrivateSlice"))));
        // UpdateUniqLocalSlice
        impl_pairs.insert((String::from("UpdateUniqLocalSlice"), TypeName::Name(String::from("UniqLocalSlice"))));
        // UpdateUniqGlobalSlice
        impl_pairs.insert((String::from("UpdateUniqGlobalSlice"), TypeName::Name(String::from("UniqGlobalSlice"))));
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
        // Trigonometric
        for s in ["Half", "Float", "Double"] {
            impl_pairs.insert((String::from("Trigonometric"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Trigonometric"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // TrigonometricExt
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("TrigonometricExt"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("TrigonometricExt"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // InvTrigonometric
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("InvTrigonometric"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("InvTrigonometric"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // InvTrigonometricExt
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("InvTrigonometricExt"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("InvTrigonometricExt"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Hyperbolic
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Hyperbolic"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Hyperbolic"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // InvHyperbolic
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("InvHyperbolic"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("InvHyperbolic"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Erf
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Erf"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Erf"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Gamma
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Gamma"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Gamma"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // LgammaR
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("LgammaR"), TypeName::Name(String::from(s))));
        }
        // LgammaRN
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("LgammaR{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Math
        for s in ["Half", "Float", "Double"] {
            impl_pairs.insert((String::from("Math"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Math"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // MathExt
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("MathExt"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("MathExt"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Frexp
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Frexp"), TypeName::Name(String::from(s))));
        }
        // FrexpN
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("Frexp{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Ilogb
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Ilogb"), TypeName::Name(String::from(s))));
        }
        // IlogbN
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("Ilogb{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Ldexp
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Ldexp"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Ldexp"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // LdexpN
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("Ldexp{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Pown
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Pown"), TypeName::Name(String::from(s))));
        }
        // PownN
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("Pown{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Remquo
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Remquo"), TypeName::Name(String::from(s))));
        }
        // RemquoN
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("Remquo{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Rootn
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Rootn"), TypeName::Name(String::from(s))));
        }
        // RootnN
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((format!("Rootn{}", n), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Common
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            impl_pairs.insert((String::from("Common"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Common"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // CommonExt
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("CommonExt"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("CommonExt"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Integer
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            impl_pairs.insert((String::from("Integer"), TypeName::Name(String::from(s))));
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("Integer"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // MadMul24
        for s in ["Int", "Uint"] {
            impl_pairs.insert((String::from("MadMul24"), TypeName::Name(String::from(s))));
        }
        for s in ["Int", "Uint"] {
            for n in [2, 3, 4, 8, 16] {
                impl_pairs.insert((String::from("MadMul24"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Cross
        for s in ["Float", "Double"] {
            for n in [3, 4] {
                impl_pairs.insert((String::from("Cross"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // HalfGeometric
        impl_pairs.insert((String::from("HalfGeometric"), TypeName::Name(String::from("Float"))));
        for n in [2, 3, 4] {
            impl_pairs.insert((String::from("HalfGeometric"), TypeName::Name(format!("Float{}", n))));
        }
        // FloatGeometric
        impl_pairs.insert((String::from("FloatGeometric"), TypeName::Name(String::from("Float"))));
        for n in [2, 3, 4] {
            impl_pairs.insert((String::from("FloatGeometric"), TypeName::Name(format!("Float{}", n))));
        }
        // DoubleGeometric
        impl_pairs.insert((String::from("DoubleGeometric"), TypeName::Name(String::from("Double"))));
        for n in [2, 3, 4] {
            impl_pairs.insert((String::from("DoubleGeometric"), TypeName::Name(format!("Double{}", n))));
        }
        // Geometric
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Geometric"), TypeName::Name(String::from(s))));
        }
        for s in ["Float", "Double"] {
            for n in [2, 3, 4] {
                impl_pairs.insert((String::from("Geometric"), TypeName::Name(format!("{}{}", s, n))));
            }
        }
        // Implementations for OpenCL.
        // HalfMath
        impl_pairs.insert((String::from("HalfMath"), TypeName::Name(String::from("Float"))));
        for n in [2, 3, 4, 8, 16] {
            impl_pairs.insert((String::from("HalfMath"), TypeName::Name(format!("Float{}", n))));
        }
        // NativeMath
        impl_pairs.insert((String::from("NativeMath"), TypeName::Name(String::from("Float"))));
        for n in [2, 3, 4, 8, 16] {
            impl_pairs.insert((String::from("NativeMath"), TypeName::Name(format!("Float{}", n))));
        }
        // FastGeometric
        impl_pairs.insert((String::from("FastGeometric"), TypeName::Name(String::from("Float"))));
        for n in [2, 3, 4] {
            impl_pairs.insert((String::from("FastGeometric"), TypeName::Name(format!("Float{}", n))));
        }
        // Relational
        for s in ["Float", "Double"] {
            impl_pairs.insert((String::from("Relational"), TypeName::Name(String::from(s))));
        }
        // RelationalIN
        for n in [2, 3, 4, 8, 16] {
            impl_pairs.insert((format!("RelationalI{}", n), TypeName::Name(format!("Float{}", n))));
        }
        // RelationalLN
        for n in [2, 3, 4, 8, 16] {
            impl_pairs.insert((format!("RelationalL{}", n), TypeName::Name(format!("Double{}", n))));
        }
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
