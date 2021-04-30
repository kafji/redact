# Redact

[![Build](https://github.com/kafji/redact/workflows/Build/badge.svg)](https://github.com/kafji/redact/actions?query=workflow%3ABuild)
[![Source](https://img.shields.io/badge/Source-666)](https://github.com/kafji/redact)

CLI application to redact sensitive keywords from a text/document.

## Usage

```
$ REDACT_KEYWORDS='["hello"]' redact 'Hello world!'
█████ world!
```

which equivalent to `echo 'Hello world!' | sed 's/hello/█████/gi'`

## Installation

```bash
cargo install --git https://github.com/kafji/redact
```

## License & Contribution

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
