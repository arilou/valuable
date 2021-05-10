use valuable::field::*;
use valuable::*;

#[test]
fn test_manual_static_impl() {
    struct MyStruct {
        num: u32,
        list: Vec<String>,
        sub: SubStruct,
    }

    static MY_STRUCT_FIELDS: &[NamedField<'static>] = &[
        NamedField::new("num"),
        NamedField::new("list"),
        NamedField::new("sub"),
    ];

    struct SubStruct {
        message: &'static str,
    }

    static SUB_STRUCT_FIELDS: &[NamedField<'static>] = &[NamedField::new("message")];

    impl Valuable for MyStruct {
        fn as_value(&self) -> Value<'_> {
            Value::Structable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_named_fields(&NamedValues::new(
                MY_STRUCT_FIELDS,
                &[
                    Value::U32(self.num),
                    Value::Listable(&self.list),
                    Value::Structable(&self.sub),
                ],
            ));
        }
    }

    impl Structable for MyStruct {
        fn definition(&self) -> StructDef<'_> {
            StructDef::new("MyStruct", Fields::NamedStatic(MY_STRUCT_FIELDS), false)
        }
    }

    impl Valuable for SubStruct {
        fn as_value(&self) -> Value<'_> {
            Value::Structable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_named_fields(&NamedValues::new(
                SUB_STRUCT_FIELDS,
                &[Value::String(self.message)],
            ));
        }
    }

    impl Structable for SubStruct {
        fn definition(&self) -> StructDef<'_> {
            StructDef::new("SubStruct", Fields::NamedStatic(SUB_STRUCT_FIELDS), false)
        }
    }

    let my_struct = MyStruct {
        num: 12,
        list: vec!["hello".to_string()],
        sub: SubStruct { message: "world" },
    };

    assert_eq!(
        format!("{:?}", my_struct.as_value()),
        "MyStruct { num: 12, list: [\"hello\"], sub: SubStruct { message: \"world\" } }"
    );
}

#[test]
fn test_named_field() {
    let name = "hello".to_string();
    let field = NamedField::new(&name[..]);
    assert_eq!(field.name(), "hello");

    let fields = [field];

    let fields = Fields::Named(&fields[..]);
    assert!(fields.is_named());
    assert!(!fields.is_unnamed());

    match fields {
        Fields::Named(..) => {}
        Fields::NamedStatic(..) | Fields::Unnamed => panic!(),
    }
}

#[test]
fn test_named_static_field() {
    static FIELDS: &[NamedField<'_>] = &[NamedField::new("hello")];

    let fields = Fields::NamedStatic(FIELDS);
    assert!(fields.is_named());
    assert!(!fields.is_unnamed());

    match fields {
        Fields::NamedStatic(..) => {}
        Fields::Named(..) | Fields::Unnamed => panic!(),
    }
}

#[test]
fn test_fields_unnamed() {
    let fields = Fields::Unnamed;
    assert!(fields.is_unnamed());
    assert!(!fields.is_named());
}

#[test]
fn test_struct_def() {
    let def = StructDef::new("hello", Fields::Unnamed, false);

    assert_eq!(def.name(), "hello");
    assert!(matches!(def.fields(), Fields::Unnamed));
    assert!(!def.is_dynamic());
}

#[test]
fn test_named_values() {
    let fields = [NamedField::new("foo"), NamedField::new("bar")];

    let vals = NamedValues::new(&fields[..], &[Value::U32(123), Value::String("hello")]);

    let other_field = NamedField::new("foo");

    assert!(matches!(vals.get(&fields[0]), Some(Value::U32(v)) if *v == 123));
    assert!(matches!(vals.get(&fields[1]), Some(Value::String(v)) if *v == "hello"));
    assert!(vals.get(&other_field).is_none());

    let e = vals.entries().collect::<Vec<_>>();
    assert_eq!(2, e.len());

    assert_eq!(e[0].0.name(), "foo");
    assert!(matches!(e[0].1, Value::U32(v) if *v == 123));

    assert_eq!(e[1].0.name(), "bar");
    assert!(matches!(e[1].1, Value::String(v) if *v == "hello"));
}