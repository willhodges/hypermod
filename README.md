Hypermod
========

An even lazier version of automod and supermod. Searches the src/ directory
recursively for .rs files, then builds a module tree using the directory names
as module names, allowing us to never write another mod statement again so long
as we live. This mimics Java package namespaces, which are also mapped to the
project directory structure.

## Syntax
In the project's main.rs or lib.rs:
```hypermod::hypermod!();```

## Example

Assume the following project directory structure:

- my_rust_project/
  - src/
    - db/
      - migrations/
        - 01-migration/
          - up.sql
          - down.sql
      - schema.rs
    - model/
      - bar.rs
      - foo.rs
    - baz.rs
    - main.rs

The call to the hypermod!() macro in main.rs or lib.rs expands to the following
mod statements and use statements:

```
mod db {
    mod schema;
    pub use self::schema::*; 
}
mod model {
    mod bar;
    pub use self::bar::*; 
    mod foo;
    pub use self::foo::*; 
}
mod baz;
pub use self::baz::*;
```

This allows the developer to skip writing mod statements and instead use any
pub item in a Rust source file through the names of the subdirectories in which
it is located. For example:

```
use crate::model::BarStruct;
use crate::db::schema_function;
```

## License

This software is licensed under either of the following:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

Any contribution you intentionally submit for inclusion in the work, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
