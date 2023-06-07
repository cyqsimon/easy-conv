mod test_newtype_wrap {
    use crate::newtype_wrap;

    #[derive(Debug, PartialEq, Eq)]
    struct Foo(String);
    newtype_wrap!(Foo, String);
    newtype_wrap!(Foo, &str);

    #[derive(Debug, PartialEq, Eq)]
    enum Bar {
        Alpha(String),
    }
    newtype_wrap!(Bar, Alpha, String);
    newtype_wrap!(Bar, Alpha, &str);

    #[test]
    fn works_on_struct() {
        let val_string = String::from("69");
        let val_str = "420";

        assert_eq!(Foo(String::from("69")), val_string.into());
        assert_eq!(Foo(String::from("420")), val_str.into());
    }

    #[test]
    fn works_on_enum() {
        let val_string = String::from("69");
        let val_str = "420";

        assert_eq!(Bar::Alpha(String::from("69")), val_string.into());
        assert_eq!(Bar::Alpha(String::from("420")), val_str.into());
    }
}

mod test_newtype_wrap_from_any {
    use crate::newtype_wrap_from_any;

    #[derive(Debug, PartialEq, Eq)]
    struct Foo(String);
    newtype_wrap_from_any!(Foo, String);

    #[derive(Debug, PartialEq, Eq)]
    enum Bar {
        Alpha(String),
    }
    newtype_wrap_from_any!(Bar, Alpha, String);

    #[test]
    fn works_on_struct() {
        let val_str = "69";

        assert_eq!(Foo(String::from("69")), val_str.into());
    }

    #[test]
    fn works_on_enum() {
        let val_str = "69";

        assert_eq!(Bar::Alpha(String::from("69")), val_str.into());
    }
}

mod test_chained_into {
    use crate::{chained_into, newtype_wrap};

    #[derive(Debug, PartialEq, Eq)]
    struct Foo(String);
    newtype_wrap!(Foo, String);

    #[derive(Debug, PartialEq, Eq)]
    struct Bar(Foo);
    newtype_wrap!(Bar, Foo);

    #[derive(Debug, PartialEq, Eq)]
    struct Baz(Bar);
    newtype_wrap!(Baz, Bar);

    chained_into!(String, Foo, Bar, Baz);

    #[test]
    fn it_works() {
        let val = String::from("69");

        assert_eq!(Baz(Bar(Foo(String::from("69")))), val.into());
    }
}
