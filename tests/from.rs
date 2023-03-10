#![allow(dead_code)]

use std::borrow::Cow;

use derive_more::From;
use static_assertions::assert_not_impl_any;

mod structs {
    use super::*;

    mod unit {
        use super::*;

        #[derive(Debug, From, PartialEq)]
        struct Unit;

        #[derive(Debug, From, PartialEq)]
        struct Tuple();

        #[derive(Debug, From, PartialEq)]
        struct Struct {}

        #[test]
        fn assert() {
            assert_eq!(Unit, ().into());
            assert_eq!(Tuple(), ().into());
            assert_eq!(Struct {}, ().into());
        }

        mod generic {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            struct Unit<const N: usize>;

            #[derive(Debug, From, PartialEq)]
            struct Tuple<const N: usize>();

            #[derive(Debug, From, PartialEq)]
            struct Struct<const N: usize> {}

            #[test]
            fn assert() {
                assert_eq!(Unit::<1>, ().into());
                assert_eq!(Tuple::<1>(), ().into());
                assert_eq!(Struct::<1> {}, ().into());
            }
        }
    }

    mod single_field {
        use super::*;

        #[derive(Debug, From, PartialEq)]
        struct Tuple(i32);

        #[derive(Debug, From, PartialEq)]
        struct Struct {
            field: i32,
        }

        #[test]
        fn assert() {
            assert_eq!(Tuple(42), 42.into());
            assert_eq!(Struct { field: 42 }, 42.into());
        }

        mod types {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            #[from(i16)]
            struct Tuple(i32);

            #[derive(Debug, From, PartialEq)]
            #[from(&str, Cow<'_, str>)]
            struct Struct {
                field: String,
            }

            #[test]
            fn assert() {
                assert_not_impl_any!(Tuple: From<i32>);
                assert_not_impl_any!(Struct: From<String>);

                assert_eq!(Tuple(42), 42_i16.into());
                assert_eq!(
                    Struct {
                        field: "42".to_string(),
                    },
                    "42".into(),
                );
                assert_eq!(
                    Struct {
                        field: "42".to_string(),
                    },
                    Cow::Borrowed("42").into(),
                );
            }
        }

        mod forward {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            #[from(forward)]
            struct Tuple(i32);

            #[derive(Debug, From, PartialEq)]
            #[from(forward)]
            struct Struct {
                field: String,
            }

            #[test]
            fn assert() {
                assert_eq!(Tuple(42), 42_i8.into());
                assert_eq!(Tuple(42), 42_i16.into());
                assert_eq!(Tuple(42), 42_i32.into());
                assert_eq!(
                    Struct {
                        field: "42".to_string(),
                    },
                    "42".into(),
                );
                assert_eq!(
                    Struct {
                        field: "42".to_string(),
                    },
                    Cow::Borrowed("42").into(),
                );
            }
        }

        mod generic {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            struct Tuple<T>(T);

            #[derive(Debug, From, PartialEq)]
            struct Struct<T> {
                field: T,
            }

            #[test]
            fn assert() {
                assert_eq!(Tuple(42), 42.into());
                assert_eq!(Struct { field: 42 }, 42.into());
            }

            mod reference {
                use super::*;

                #[derive(Debug, From, PartialEq)]
                struct Tuple<'a, T>(&'a T);

                #[derive(Debug, From, PartialEq)]
                struct Struct<'a, T> {
                    field: &'a T,
                }

                #[test]
                fn assert() {
                    assert_eq!(Tuple(&42), (&42).into());
                    assert_eq!(Struct { field: &42 }, (&42).into());
                }
            }

            mod indirect {
                use super::*;

                #[derive(Debug, From, PartialEq)]
                struct Tuple<T: 'static>(&'static T);

                #[derive(Debug, From, PartialEq)]
                struct Struct<T: 'static> {
                    field: &'static T,
                }

                #[test]
                fn assert() {
                    assert_eq!(Tuple(&42), (&42).into());
                    assert_eq!(Struct { field: &42 }, (&42).into());
                }
            }

            mod bounded {
                use super::*;

                #[derive(Debug, From, PartialEq)]
                struct Tuple<T: Clone>(T);

                #[derive(Debug, From, PartialEq)]
                struct Struct<T: Clone> {
                    field: T,
                }

                #[test]
                fn assert() {
                    assert_eq!(Tuple(42), 42.into());
                    assert_eq!(Struct { field: 42 }, 42.into());
                }
            }

            mod r#const {
                use super::*;

                #[derive(Debug, From, PartialEq)]
                struct Tuple<const N: usize, T>(T);

                #[derive(Debug, From, PartialEq)]
                struct Struct<T, const N: usize> {
                    field: T,
                }

                #[test]
                fn assert() {
                    assert_eq!(Tuple::<1, _>(1), 1.into());
                    assert_eq!(Struct::<_, 1> { field: 1 }, 1.into());
                }
            }
        }
    }

    mod multi_field {
        use super::*;

        #[derive(Debug, From, PartialEq)]
        struct Tuple(i32, i16);

        #[derive(Debug, From, PartialEq)]
        struct Struct {
            field1: i32,
            field2: i16,
        }

        #[test]
        fn assert() {
            assert_eq!(Tuple(0, 1), (0, 1_i16).into());
            assert_eq!(
                Struct {
                    field1: 0,
                    field2: 1,
                },
                (0, 1_i16).into(),
            );
        }

        mod types {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            #[from((i16, i16))]
            struct Tuple(i32, i16);

            #[derive(Debug, From, PartialEq)]
            #[from((i16, i16))]
            struct Struct {
                field1: i32,
                field2: i16,
            }

            #[test]
            fn assert() {
                assert_not_impl_any!(Tuple: From<(i32, i16)>);
                assert_not_impl_any!(Struct: From<(i32, i16)>);

                assert_eq!(Tuple(0, 1), (0_i16, 1_i16).into());
                assert_eq!(
                    Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i16, 1_i16).into(),
                );
            }
        }

        mod forward {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            #[from(forward)]
            struct Tuple(i32, i16);

            #[derive(Debug, From, PartialEq)]
            #[from(forward)]
            struct Struct {
                field1: i32,
                field2: i16,
            }

            #[test]
            fn assert() {
                assert_eq!(Tuple(0, 1), (0_i8, 1_i8).into());
                assert_eq!(Tuple(0, 1), (0_i8, 1_i16).into());
                assert_eq!(Tuple(0, 1), (0_i16, 1_i8).into());
                assert_eq!(Tuple(0, 1), (0_i16, 1_i16).into());
                assert_eq!(Tuple(0, 1), (0_i32, 1_i8).into());
                assert_eq!(Tuple(0, 1), (0_i32, 1_i16).into());
                assert_eq!(
                    Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i8, 1_i8).into(),
                );
                assert_eq!(
                    Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i8, 1_i16).into(),
                );
                assert_eq!(
                    Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i16, 1_i8).into(),
                );
                assert_eq!(
                    Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i16, 1_i16).into(),
                );
                assert_eq!(
                    Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i32, 1_i8).into(),
                );
                assert_eq!(
                    Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i32, 1_i16).into(),
                );
            }
        }

        mod generic {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            struct Tuple<A, B>(A, B);

            #[derive(Debug, From, PartialEq)]
            struct Struct<A, B> {
                field1: A,
                field2: B,
            }

            #[test]
            fn assert() {
                assert_eq!(Tuple(1, 2_i8), (1, 2_i8).into());
                assert_eq!(
                    Struct {
                        field1: 1,
                        field2: 2_i8,
                    },
                    (1, 2_i8).into(),
                );
            }

            mod reference {
                use super::*;

                #[derive(Debug, From, PartialEq)]
                struct Tuple<'a, A, B>(&'a A, &'a B);

                #[derive(Debug, From, PartialEq)]
                struct Struct<'a, A, B> {
                    field1: &'a A,
                    field2: &'a B,
                }

                #[test]
                fn assert() {
                    assert_eq!(Tuple(&1, &2_i8), (&1, &2_i8).into());
                    assert_eq!(
                        Struct {
                            field1: &1,
                            field2: &2_i8,
                        },
                        (&1, &2_i8).into(),
                    );
                }
            }

            mod bounded {
                use super::*;

                #[derive(Debug, From, PartialEq)]
                struct Tuple<A: Clone, B>(A, B);

                #[derive(Debug, From, PartialEq)]
                struct Struct<A: Clone, B> {
                    field1: A,
                    field2: B,
                }

                #[test]
                fn assert() {
                    assert_eq!(Tuple(1, 2_i8), (1, 2_i8).into());
                    assert_eq!(
                        Struct {
                            field1: 1,
                            field2: 2_i8,
                        },
                        (1, 2_i8).into(),
                    );
                }
            }

            mod r#const {
                use super::*;

                #[derive(Debug, From, PartialEq)]
                struct ConstTuple<const N: usize, A, B>(A, B);

                #[derive(Debug, From, PartialEq)]
                struct ConstStruct<const N: usize, A, B> {
                    field1: A,
                    field2: B,
                }

                #[test]
                fn assert() {
                    assert_eq!(ConstTuple::<1, _, _>(1, 2_i8), (1, 2_i8).into());
                    assert_eq!(
                        ConstStruct::<1, _, _> {
                            field1: 1,
                            field2: 2_i8,
                        },
                        (1, 2_i8).into(),
                    );
                }
            }
        }
    }
}

mod enums {
    use super::*;

    mod unit_variant {
        use super::*;

        #[derive(Debug, From, PartialEq)]
        enum Enum {
            #[from]
            Unit,
            Unnamed(),
            Named {},
        }

        #[test]
        fn assert() {
            assert_eq!(Enum::Unit, ().into());
        }

        mod generic {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Enum<const N: usize> {
                #[from]
                Unit,
                Unnamed(),
                Named {},
            }

            #[test]
            fn assert() {
                assert_eq!(Enum::<0>::Unit, ().into());
            }
        }
    }

    mod single_field_variant {
        use super::*;

        #[derive(Debug, From, PartialEq)]
        enum Enum {
            Unnamed(i8),
            Named { field: i16 },
        }

        #[test]
        fn assert() {
            assert_eq!(Enum::Unnamed(1), 1_i8.into());
            assert_eq!(Enum::Named { field: 1 }, 1_i16.into());
        }

        mod skip {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Enum {
                Unnamed(i8),
                #[from(skip)]
                UnnamedSkipped(i8),
                Named {
                    field: i16,
                },
                #[from(skip)]
                NamedSkipped {
                    field: i16,
                },
            }

            #[test]
            fn assert() {
                assert_eq!(Enum::Unnamed(1), 1_i8.into());
                assert_eq!(Enum::Named { field: 1 }, 1_i16.into());
            }
        }

        mod types {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Enum {
                #[from(i8)]
                Unnamed(i16),
                #[from(i16)]
                Named {
                    field: i32,
                },
                NoAttribute(i32),
            }

            #[test]
            fn assert() {
                assert_eq!(Enum::Unnamed(1), 1_i8.into());
                assert_eq!(Enum::Named { field: 1 }, 1_i16.into());
                assert_eq!(Enum::NoAttribute(1), 1.into());
            }
        }

        mod forward {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Unnamed {
                #[from(forward)]
                Variant(i32),
                AutomaticallyIgnored(i32),
            }

            #[derive(Debug, From, PartialEq)]
            enum Named {
                #[from(forward)]
                Variant {
                    field: i32,
                },
                AutomaticallyIgnored {
                    field: i32,
                },
            }

            #[test]
            fn assert() {
                assert_eq!(Unnamed::Variant(1), 1_i8.into());
                assert_eq!(Unnamed::Variant(1), 1_i16.into());
                assert_eq!(Unnamed::Variant(1), 1_i32.into());
                assert_eq!(Named::Variant { field: 1 }, 1_i8.into());
                assert_eq!(Named::Variant { field: 1 }, 1_i16.into());
                assert_eq!(Named::Variant { field: 1 }, 1_i32.into());
            }
        }

        mod generic {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Tuple<T> {
                Variant(T),
                #[from(skip)]
                Skipped(T),
            }

            #[derive(Debug, From, PartialEq)]
            enum Struct<T> {
                Variant {
                    field: T,
                },
                #[from(skip)]
                Skipped {
                    field: T,
                },
            }

            #[derive(Debug, From, PartialEq)]
            enum RefTuple<'a, T> {
                Variant(&'a T),
                #[from(skip)]
                Skipped(&'a T),
            }

            #[derive(Debug, From, PartialEq)]
            enum RefStruct<'a, T> {
                Variant {
                    field: &'a T,
                },
                #[from(skip)]
                Skipped {
                    field: &'a T,
                },
            }

            #[derive(Debug, From, PartialEq)]
            enum BoundedTuple<T: Clone> {
                Variant(T),
                #[from(skip)]
                Skipped(T),
            }

            #[derive(Debug, From, PartialEq)]
            enum BoundedStruct<T: Clone> {
                Variant {
                    field: T,
                },
                #[from(skip)]
                Skipped {
                    field: T,
                },
            }

            #[derive(Debug, From, PartialEq)]
            enum ConstTuple<T, const N: usize> {
                Variant(T),
                #[from(skip)]
                Skipped(T),
            }

            #[derive(Debug, From, PartialEq)]
            enum ConstStruct<const N: usize, T> {
                Variant {
                    field: T,
                },
                #[from(skip)]
                Skipped {
                    field: T,
                },
            }

            #[test]
            fn assert() {
                assert_eq!(Tuple::Variant(1), 1.into());
                assert_eq!(Struct::Variant { field: 1 }, 1.into());
                assert_eq!(RefTuple::Variant(&1), (&1).into());
                assert_eq!(RefStruct::Variant { field: &1 }, (&1).into());
                assert_eq!(BoundedTuple::Variant(1), 1.into());
                assert_eq!(BoundedStruct::Variant { field: 1 }, 1.into());
                assert_eq!(ConstTuple::Variant::<_, 1>(1), 1.into());
                assert_eq!(ConstStruct::Variant::<1, _> { field: 1 }, 1.into());
            }
        }
    }

    mod multi_field_variant {
        use super::*;

        #[derive(Debug, From, PartialEq)]
        enum Enum {
            Tuple(i8, i8),
            Struct { field1: i16, field2: i16 },
        }

        #[test]
        fn assert() {
            assert_eq!(Enum::Tuple(0, 1), (0_i8, 1_i8).into());
            assert_eq!(
                Enum::Struct {
                    field1: 0,
                    field2: 1
                },
                (0_i16, 1_i16).into(),
            );
        }

        mod skip {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Enum {
                Tuple(i8, i8),
                #[from(skip)]
                TupleSkipped(i8, i8),
                Struct {
                    field1: i16,
                    field2: i16,
                },
                #[from(skip)]
                StructSkipped {
                    field1: i16,
                    field2: i16,
                },
            }

            #[test]
            fn assert() {
                assert_eq!(Enum::Tuple(0, 1), (0_i8, 1_i8).into());
                assert_eq!(
                    Enum::Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i16, 1_i16).into(),
                );
            }
        }

        mod types {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Enum {
                #[from((i8, i8))]
                Tuple(i16, i16),
                #[from((i16, i16))]
                Struct {
                    field1: i32,
                    field2: i32,
                },
                StructNoAttribute {
                    field1: i32,
                    field2: i32,
                },
            }

            #[test]
            fn assert() {
                assert_eq!(Enum::Tuple(0, 1), (0_i8, 1_i8).into());
                assert_eq!(
                    Enum::Struct {
                        field1: 0,
                        field2: 1
                    },
                    (0_i16, 1_i16).into(),
                );
                assert_eq!(
                    Enum::StructNoAttribute {
                        field1: 0,
                        field2: 1
                    },
                    (0_i32, 1_i32).into(),
                );
            }
        }

        mod forward {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Unnamed {
                #[from(forward)]
                Variant(i16, i16),
                AutomaticallyIgnored(i16, i16),
            }

            #[derive(Debug, From, PartialEq)]
            enum Named {
                #[from(forward)]
                Variant {
                    field1: i16,
                    field2: i16,
                },
                AutomaticallyIgnored {
                    field1: i16,
                    field2: i16,
                },
            }

            #[test]
            fn assert() {
                assert_eq!(Unnamed::Variant(0, 1), (0_i8, 1_i8).into());
                assert_eq!(Unnamed::Variant(0, 1), (0_i8, 1_i16).into());
                assert_eq!(Unnamed::Variant(0, 1), (0_i16, 1_i8).into());
                assert_eq!(Unnamed::Variant(0, 1), (0_i16, 1_i16).into());
                assert_eq!(
                    Named::Variant {
                        field1: 0,
                        field2: 1
                    },
                    (0_i8, 1_i8).into(),
                );
                assert_eq!(
                    Named::Variant {
                        field1: 0,
                        field2: 1
                    },
                    (0_i8, 1_i16).into(),
                );
                assert_eq!(
                    Named::Variant {
                        field1: 0,
                        field2: 1
                    },
                    (0_i16, 1_i8).into(),
                );
                assert_eq!(
                    Named::Variant {
                        field1: 0,
                        field2: 1
                    },
                    (0_i16, 1_i16).into(),
                );
            }
        }

        mod generic {
            use super::*;

            #[derive(Debug, From, PartialEq)]
            enum Tuple<A, B> {
                Variant(A, B),
                #[from(skip)]
                Skipped(A, B),
            }

            #[derive(Debug, From, PartialEq)]
            enum Struct<A, B> {
                Variant {
                    field1: A,
                    field2: B,
                },
                #[from(skip)]
                Skipped {
                    field1: A,
                    field2: B,
                },
            }

            #[derive(Debug, From, PartialEq)]
            enum RefTuple<'a, A, B> {
                Variant(&'a A, &'a B),
                #[from(skip)]
                Skipped(&'a A, &'a B),
            }

            #[derive(Debug, From, PartialEq)]
            enum RefStruct<'a, A, B> {
                Variant {
                    field1: &'a A,
                    field2: &'a B,
                },
                #[from(skip)]
                Skipped {
                    field1: &'a A,
                    field2: &'a B,
                },
            }

            #[derive(Debug, From, PartialEq)]
            enum BoundedTuple<A: Clone, B> {
                Variant(A, B),
                #[from(skip)]
                Skipped(A, B),
            }

            #[derive(Debug, From, PartialEq)]
            enum BoundedStruct<A, B: Clone> {
                Variant {
                    field1: A,
                    field2: B,
                },
                #[from(skip)]
                Skipped {
                    field1: A,
                    field2: B,
                },
            }

            #[derive(Debug, From, PartialEq)]
            enum ConstTuple<const N: usize, A, B> {
                Variant(A, B),
                #[from(skip)]
                Skipped(A, B),
            }

            #[derive(Debug, From, PartialEq)]
            enum ConstStruct<A, const N: usize, B> {
                Variant {
                    field1: A,
                    field2: B,
                },
                #[from(skip)]
                Skipped {
                    field1: A,
                    field2: B,
                },
            }

            #[test]
            fn assert() {
                assert_eq!(Tuple::Variant(1, 2_i16), (1, 2_i16).into());
                assert_eq!(
                    Struct::Variant {
                        field1: 1,
                        field2: 2_i16,
                    },
                    (1, 2_i16).into(),
                );
                assert_eq!(RefTuple::Variant(&1, &2_i16), (&1, &2_i16).into());
                assert_eq!(
                    RefStruct::Variant {
                        field1: &1,
                        field2: &2_i16,
                    },
                    (&1, &2_i16).into(),
                );
                assert_eq!(BoundedTuple::Variant(1, 2_i16), (1, 2_i16).into());
                assert_eq!(
                    BoundedStruct::Variant {
                        field1: 1,
                        field2: 2_i16,
                    },
                    (1, 2_i16).into(),
                );
                assert_eq!(ConstTuple::Variant::<0, _, _>(1, 2_i16), (1, 2_i16).into());
                assert_eq!(
                    ConstStruct::<_, 1, _>::Variant {
                        field1: 1,
                        field2: 2_i16,
                    },
                    (1, 2_i16).into(),
                );
            }
        }
    }
}
