If you are new to GitHub, visit the
[first-contributions instructions](https://github.com/firstcontributions/first-contributions/blob/master/README.md)
to learn how to contribute on GitHub.

If you are new to Rust, I recommend you to go through
[the book](https://doc.rust-lang.org/book).

Development Guideline
---------------------

Assuming that you have mentioned the issue you are working on and that you have
forked and cloned the repository locally, in order to contribute by making
changes to the code follow the steps below:

### Make changes to the code

The project is divided into three crates:

- The local web server: `axum-server`
- The cloudflare worker: `cf-worker`
- The common libraries: `libs`

To add or fix something, you might want to start at `libs`. Then validate and
test it locally using `axum-server`.

```bash
cargo run --bin axum-server
```

You can also use [cargo-watch](https://github.com/passcod/cargo-watch) to test
your changes in real-time.

```bash
# Install cargo watch
cargo install cargo-watch

# Run
cargo watch -- cargo run --bin axum-server
```

Then you can come to `cf-worker` and implement the changes. Test & validate
using [wrangler](https://github.com/cloudflare/wrangler)

```bash

# Install cargo watch
cargo install wrangler

# Run
wrangler dev
```

### Format code and get linting helps

It's recommended to format your code and run linter before you push. Run the
following commands.

```bash
cargo fmt

cargo clippy
```

### Commit, push and finally create a pull request.

Don't worry if you make a mistake, we will provide constructive feedback and
guidance to improve the pull request.

If you encounter any situation that violates our
[code of conduct](https://github.com/sayanarijit/qrcode.show/blob/main/CODE_OF_CONDUCT.md)
please report it to sayanarijit@gmail.com.
