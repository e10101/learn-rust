# Learn Rust

## Setup Hugo

You can use following command to install Hugo.

```bash
# For MacOS
brew install hugo
```

Using following command to check the version of Hugo.

```bash
hugo version
```

You will see the version information of Hugo. For example,

```
hugo v0.134.2+extended darwin/amd64 BuildDate=2024-09-10T10:46:33Z VendorInfo=brew
```

## Install Git submodule

```bash
git submodule update --init --recursive
```

## Run Hugo Server

You can use following command to run the Hugo server.

```bash
hugo server --minify --port 8080
```

Then open your browser and navigate to `http://localhost:8080`.