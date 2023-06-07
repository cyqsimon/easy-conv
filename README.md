# easy-conv

Cut down on trivial `impl From<A> for B` boilerplate code.

[crates.io](https://crates.io/crates/easy-conv)

## Quick start

Ever tired of code that looks like this?

```rust
enum Choices {
    Alpha(TypeA),
    Bravo(TypeB),
    Charlie(TypeC),
}
impl From<TypeA> for Choices {
    fn from(val: TypeA) -> Self {
        Choices::Alpha(val)
    }
}
impl From<TypeB> for Choices {
    fn from(val: TypeB) -> Self {
        Choices::Bravo(val)
    }
}
impl From<TypeC> for Choices {
    fn from(val: TypeC) -> Self {
        Choices::Charlie(val)
    }
}
```

How about this?

```rust
enum Choices {
    Alpha(TypeA),
    Bravo(TypeB),
    Charlie(TypeC),
}
newtype_wrap!(Choices, Alpha, TypeA);
newtype_wrap!(Choices, Bravo, TypeB);
newtype_wrap!(Choices, Charlie, TypeC);
```

Better? I certainly think so.

There are also `newtype_wrap_from_any` and `chained_into` macros, which each does
their quirky little thing. See [documentation](https://docs.rs/easy-conv/latest/).

## Contributing

If you have any other type conversion needs that involve a lot of boilerplate code,
please feel free to raise an issue (or better yet, a PR). Maybe we can add a macro
to make life easier for you and everyone else.
