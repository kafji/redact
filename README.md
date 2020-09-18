# Redact

[![Build](https://github.com/kafji/redact/workflows/Build/badge.svg)](https://github.com/kafji/redact/actions?query=workflow%3ABuild)

CLI application to redact sensitive keywords from a text/document.

### Usage

```
$ REDACT_KEYWORDS='["hello"]' redact 'Hello world!'
█████ world!
```

which equivalent to `echo 'Hello world!' | sed 's/hello/█████/gi'`

### Installation

Install using `cargo install` command and set env var `REDACT_KEYWORDS` to a list of keywords to be redacted in JSON array format e.g. `export REDACT_KEYWORDS='["secret", "private"]'`

<br>

### License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
