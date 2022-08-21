the main exposed function of the crate is `parse(bytes: &[u8]) -> Circuit`

example of usage:

```
use tc_save_monger::{parse, Circuit};
use std::fs;

fn parse_from_file(path: &str) -> Circuit {
    let bytes = fs::read(path).expect("error reading file");
    parse(&bytes)
}
```