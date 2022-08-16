// #![allow(dead_code)]
//
// use std::{collections::HashMap, sync::Arc};
//
// use anyhow::Result;
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub(crate) enum PrimitiveTypes {
//     SignedVoid,
//     SignedChar,
//     SignedShort,
//     SignedInt,
//     SignedLongInt,
//     SignedLongLongInt,
//     SignedFloat,
//     SignedDouble,
//     SignedLongDouble,
//     UnsignedVoid,
//     UnsignedChar,
//     UnsignedShort,
//     UnsignedInt,
//     UnsignedLongInt,
//     UnsignedLongLongInt,
//     UnsignedFloat,
//     UnsignedDouble,
//     UnsignedLongDouble,
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub(crate) enum DerivedType {
//     Array(Arc<Type>, usize),
//     Structure(HashMap<&'static str, Type>),
//     Union(HashMap<&'static str, Type>),
//     Function(Vec<Type>, Arc<Type>),
//     Pointer(Arc<Type>),
//     Primitive(PrimitiveTypes),
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub(crate) enum QualifiedTypeTag {
//     Const,
//     Volatile,
//     Restrict,
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub(crate) struct Type {
//     pub(crate) qualifier: Option<QualifiedTypeTag>,
//     pub(crate) derived_type: DerivedType,
// }
//
// fn parse_type(type_str: &str) -> Result<(Option<&str>, Type)> {
//     todo!()
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::ast::types::{parse_type, DerivedType, PrimitiveTypes, QualifiedTypeTag, Type};
//
//     #[test]
//     fn test_parse_type() {
//         // int -> Type(None, Primitive(SignedInt))
//         // const int -> Type(Const, Primitive(SignedInt))
//         // const -> None
//         // const int * -> Type(None, Pointer(Type(Const, Primitive(SignedInt))))
//         // int const * -> Type(None, Pointer(Type(Const, Primitive(SignedInt))))
//         // const float int -> None
//         // int * const -> Type(Const, Pointer(Type(None, Primitive(SignedInt))))
//         // int * const * -> Type(None, Pointer(Type(Const, Pointer(Type(None,
//         // Primitive(SignedInt))))))
//         // int[4] -> Type(None, Array(Type(None, Primitive(SignedInt)), 4))
//         // unsigned int -> Type(None, Primitive(UnsignedInt))
//         // unsigned int[4] -> Type(None, Array(Type(None, Primitive(UnsignedInt)), 4))
//
//         // Bad Inputs
//         assert!(parse_type("const").is_err());
//         assert!(parse_type("const float int").is_err());
//
//         // Good inputs
//         assert_eq!(
//             parse_type("int").ok(),
//             Some((
//                 None,
//                 Type {
//                     qualifier: None,
//                     derived_type: DerivedType::Primitive(PrimitiveTypes::SignedInt)
//                 }
//             ))
//         );
//     }
// }
