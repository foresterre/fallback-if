# fallback-if

Find a common root subdirectory for a set of input paths.
Optionally, concatenate the paths which share this common root subdirectory to a new to a new common root.

For example, given:

```
/my/my/common/path/a.png
/my/my/common/path/b.png
/my/my/uncommon/path/c.png
```

It finds the common root: `/my/my`, and the unrooted branches:

```
common/path/a.png
common/path/b.png
uncommon/path/c.png
```

If you decide to concatenate a new root directory `/new`, it will output:

```
/new/common/path/a.png
/new/common/path/b.png
/new/uncommon/path/c.png
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
