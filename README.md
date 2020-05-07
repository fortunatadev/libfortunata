# Libfortunata

Fortunata is a multipurpose application patcher and launcher that expands on the design of [Tequila](https://github.com/leandrotlz/Tequila) and [Cream Soda](https://github.com/thunderspynetwork/creamsoda).

Fortunata uses an entirely rebuilt Rust codebase designed to be both more efficient and more secure by supporting parallel downloads, deduping files where possible, and enforcing checksumming and patching over https.

## Fortunata Library

Libfortunata provides a low-level ABI for interfacing with Fortunata-based applications. If you're looking for the first-party Fortunata patcher/launcher GUI, look at [Fortunata Launcher](https://github.com/fortunatadev/fortunata-launcher).

TODO: Link to Rust docs when published.

TODO: Binary distributions.

## Manifest Files

Fortunata uses Manifest files to determine what files to download and update. Tequila-format XML manifests are supported, but not recommended.

Fortunata's native Manifest format is stored as [TOML](https://github.com/toml-lang/toml), which enforces a more regular and easily parsable data structure than XML. An [example manifest](https://github.com/fortunatadev/fortunata-patcher/blob/master/examples/Manifest.toml) is available for reference.

A CLI tool, [Manifesto](https://github.com/fortunatadev/fortunata-manifesto), is also available for application admins to generate and manage Manifest files. Manifesto can also convert Tequila XML manifests to Fortunata manifests.

## Configuration

Libfortunata supports reading and writing `fortunata.toml` files used by the Fortunata launcher for configuration. Alternatively, config may be passed in at runtime.

## Future Plans

Fortunata is still in early development and unstable.

A rough roadmap of planned development in order of priority is provided below.

* Expanded test coverage
* Makefiles and build pipelines
* Mirror ranking
* Associating files with profiles / selectively patching specific profiles *(breaks Tequila compatability)*

## Building

Fortunata can be built locally using the standard stable Rust toolchain and `cargo build --lib`.

Libfortunata may also be built as an application binary, which will patch based off a `fortunata.toml` config file in the project root if one is provided.

This binary may be used as a background patcher if desired, although it does not provide any method for lifecycle management or state reporting.

## Contributing

Contributions are welcome, especially targeted at any of the Future Plans goals.

Please ensure that your code conforms to Rust style guidelines (`cargo fmt`) and that existing tests are passing (`cargo test`).

Pull requests without test coverage are discourage but will be considered, given the incomplete and unstable nature of the project.