---
weight: 1
bookFlatSection: true
title: "Axum Basic"
---

# Axum Basic

## Test without refreshing web browser

We can create a test file under `tests` directory.

But before that, we need to install `cargo-watch` to automatically run the test when the code is changed.

```bash
cargo install cargo-watch
```

So we can watch the main application changes by following command:

```shell
cargo watch -q -c -w src/ -x run
```

In the above code:

- The `-q` option is for quiet mode, which will not print the log.
- The `-c` option is for continuous mode, which will automatically run the test when the code is changed.
- The `-w` option is for watching the `src` directory.
- The `-x` option is for running the `run` command. 

About the test part, we can run following command to run the test:

```shell
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

In the above code, the `--nocapture` option is used to prevent the test from being captured by the terminal.

Therefore, when the main application has any changes, we will auto refresh the API's behavior. At the same time, the test changes will be also captured and tested.