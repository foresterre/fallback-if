# fallback-if

Fallback to an alternative, given that the initial result is considered a **fail** and the predicate
evaluates to **true**.

## Example

```rust
fn main() {
    struct Config {
        fallback_to_local: bool,
    }

    #[derive(Debug, PartialEq)]
    struct Manifest;

    impl Manifest {
        pub fn try_fetch_remote() -> Result<Self, ()> {
            // Oh noes! failed to fetch manifest remotely
            Err(())
        }

        pub fn try_fetch_local() -> Result<Self, ()> {
            // Yesss! Fetched locally!
            Ok(Manifest)
        }
    }

    let config = Config { fallback_to_local: true };
    let result = Manifest::try_fetch_remote();

    let outcome = result.fallback_if(config.fallback_to_local, || {
        Manifest::try_fetch_local()
    });

    assert_eq!(outcome, Ok(Manifest))
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be licensed as above, without any additional terms or
conditions.
