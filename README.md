# Learn Rust

## Links

- Dev: [https://learn-rust-ekm.pages.dev/](https://learn-rust-ekm.pages.dev/)

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
hugo server --minify --port 8812
```

Then open your browser and navigate to [http://localhost:8080](http://localhost:8080).

## Create a new section or post

### Create a new section

To create a new section, use the following command:

```bash
hugo new --kind chapter content/<section-name>/_index.md
```

Replace `<section-name>` with the desired name for your new section. For example:

```bash
hugo new --kind chapter content/advanced-topics/_index.md
```

This will create a new section called "Advanced Topics" with an `_index.md` file.

### Create a new post

To create a new post within a section, use the following command:

```bash
hugo new content/<section-name>/<post-name>.md
```

Replace `<section-name>` with the name of the section where you want to add the post, and `<post-name>` with the desired name for your new post. For example:

```bash
hugo new content/axum/middleware.md
```

This will create a new post called "middleware.md" in the "axum" section.

After creating a new section or post, you can edit the generated Markdown file to add your content.
