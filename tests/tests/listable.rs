use tests::*;
use valuable::*;

struct VisitHello(u32);

impl Visit for VisitHello {
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        let id = &HELLO_WORLD_FIELDS[0];
        assert_eq!("id", id.name());
        assert_eq!(Some(self.0), named_values.get(id).unwrap().as_u32());
    }
}

#[derive(Default)]
struct VisitList(u32);

impl Visit for VisitList {
    fn visit_slice(&mut self, slice: Slice<'_>) {
        match slice {
            Slice::Value(slice) => {
                for value in slice {
                    let value = value.as_structable().unwrap();

                    // Check only one visit method is called
                    let counts = tests::visit_counts(&value);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_named_fields: 1,
                            ..Default::default()
                        }
                    );

                    // Check the next ID
                    let mut v = VisitHello(self.0);
                    value.visit(&mut v);
                    self.0 += 1;
                }
            }
            _ => panic!(),
        }
    }
}

#[test]
fn test_default_visit_slice_empty() {
    let empty: Vec<HelloWorld> = vec![];

    assert_eq!(Listable::size_hint(&empty), (0, Some(0)));

    let counts = tests::visit_counts(&empty);
    assert_eq!(
        counts,
        tests::VisitCount {
            visit_slice: 1,
            ..Default::default()
        }
    );
}

#[test]
fn test_default_visit_slice_small() {
    let hellos = (0..4).map(|i| HelloWorld { id: i }).collect::<Vec<_>>();

    assert_eq!(Listable::size_hint(&hellos), (4, Some(4)));

    let counts = tests::visit_counts(&hellos);
    assert_eq!(
        counts,
        tests::VisitCount {
            visit_slice: 1,
            ..Default::default()
        }
    );

    let mut visit = VisitList::default();
    hellos.visit(&mut visit);
    assert_eq!(visit.0, 4);
}

#[test]
fn test_default_visit_slice_big_pow_2() {
    let hellos = (0..1024).map(|i| HelloWorld { id: i }).collect::<Vec<_>>();

    assert_eq!(Listable::size_hint(&hellos), (1024, Some(1024)));

    let counts = tests::visit_counts(&hellos);
    assert_eq!(
        counts,
        tests::VisitCount {
            visit_slice: 128,
            ..Default::default()
        }
    );

    let mut visit = VisitList::default();
    hellos.visit(&mut visit);
    assert_eq!(visit.0, 1024);
}

#[test]
fn test_default_visit_slice_big_odd() {
    let hellos = (0..63).map(|i| HelloWorld { id: i }).collect::<Vec<_>>();

    assert_eq!(Listable::size_hint(&hellos), (63, Some(63)));

    let counts = tests::visit_counts(&hellos);
    assert_eq!(
        counts,
        tests::VisitCount {
            visit_slice: 8,
            ..Default::default()
        }
    );

    let mut visit = VisitList::default();
    hellos.visit(&mut visit);
    assert_eq!(visit.0, 63);
}

macro_rules! test_primitive {
    (
        $(
            $name:ident, $variant:ident: $ty:ty => |$x:ident| $b:block;
        )*
    ) => {
        $(
            mod $name {
                use valuable::*;

                struct VisitPrimitive(Vec<$ty>);

                impl Visit for VisitPrimitive {
                    fn visit_slice(&mut self, slice: Slice<'_>) {
                        match slice {
                            Slice::$variant(slice) => {
                                assert_eq!(slice, &self.0[..]);
                            }
                            _ => panic!(),
                        }
                    }
                }

                #[test]
                fn test_empty() {
                    let empty: Vec<$ty> = vec![];

                    assert_eq!(Listable::size_hint(&empty), (0, Some(0)));

                    let counts = tests::visit_counts(&empty);
                    assert_eq!(counts, tests::VisitCount { visit_slice: 1, .. Default::default() });
                }

                #[test]
                fn test_slices() {
                    for &len in &[4_usize, 10, 30, 32, 63, 64, 100, 1000, 1024] {
                        let vec = (0..len).map(|$x| $b).collect::<Vec<$ty>>();

                        assert_eq!(Listable::size_hint(&vec), (len, Some(len)));

                        let counts = tests::visit_counts(&vec);
                        assert_eq!(counts, tests::VisitCount { visit_slice: 1, .. Default::default() });

                        let mut visit = VisitPrimitive(vec.clone());
                        vec.visit(&mut visit);
                    }
                }
            }
        )*
    };
}

test_primitive! {
    test_bool, Bool: bool => |x| { x % 2 == 0 };
    test_char, Char: char => |x| { core::convert::TryFrom::try_from(x as u32).unwrap_or('f') };
    test_f32, F32: f32 => |x| { x as f32 };
    test_f64, F64: f64 => |x| { x as f64 };
    test_i8, I8: i8 => |x| { x as i8 };
    test_i16, I16: i16 => |x| { x as i16 };
    test_i32, I32: i32 => |x| { x as i32 };
    test_i64, I64: i64 => |x| { x as i64 };
    test_i128, I128: i128 => |x| { x as i128 };
    test_isize, Isize: isize => |x| { x as isize };
    test_str, Str: &'static str => |x| { crate::leak(format!("{}", x)) };
    test_string, String: String => |x| { format!("{}", x) };
    test_u8, U8: u8 => |x| { x as u8 };
    test_u16, U16: u16 => |x| { x as u16 };
    test_u32, U32: u32 => |x| { x as u32 };
    test_u64, U64: u64 => |x| { x as u64 };
    test_u128, U128: u128 => |x| { x as u128 };
    test_usize, Usize: usize => |x| { x as usize };
    test_unit, Unit: () => |_x| { () };
}

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}