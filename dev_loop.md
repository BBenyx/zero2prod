## Developement loop from zero2prod
* Faster compilation with cargo watch
  * Command: **cargo watch -x check**
  * If test running also needed: **cargo watch -x check -x test -x run**
* Monitoring code coverage with tarpaulin
  * Command: **cargo tarpaulin --ignore-tests**
* Linting to try discovering unidiomatic code
  * Command: **cargo clippy -- -D warnings**
  * It will fail if unidiomatic part was found
  * Muting for part where the suggestion is not important **#[allow(clippy::lint_name)]**
* Auto formatting
  * Command: **cargo fmt -- --check**
  * This will fail the commit if bad formatting was found
  * Auto format with: **cargo fmt**
* Tree scan for reported crate vulnerabilities
  * Command: **cargo audit**