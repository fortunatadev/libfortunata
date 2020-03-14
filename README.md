# Vanguard Patcher

Vanguard is a multipurpose application patcher and launcher that expands on the design of [Tequila](https://github.com/leandrotlz/Tequila) and [Cream Soda](https://github.com/thunderspynetwork/creamsoda).

Vanguard uses an entirely rebuilt Rust codebase designed to be both more efficient and more secure by supporting parallel downloads, deduping files where possible, and enforcing checksumming and patching over https.

## Installation

Vanguard patcher is not normally installed and used on its own, but as a backend for the [Vanguard Launcher](https://github.com/vanguarddev/vanguard-launcher). Please consult the information that is (hopefully) provided by application teams for information on how to download and use the launcher.

Binary distributions will be provided, but they will not be configured for any specific application.

## Configuration

The `vanguard.toml` file in the project root may be used for configuration. Ordinarily, configuration will be handled by the Launcher rather than by direct file edits.

## Manifest Files

Vanguard uses Manifest files to determine what files to download and update. Tequila-format XML manifests are supported, but not recommended.

Vanguard's native Manifest format is stored as [TOML](https://github.com/toml-lang/toml), which enforces a more regular and easily parsable data structure than XML. An [example manifest](https://github.com/vanguarddev/vanguard-patcher/blob/master/examples/Manifest.toml) is available for reference.

A CLI tool, [Manifesto](https://github.com/vanguarddev/vanguard-manifesto), is also available for application admins to generate and manage Manifest files. Manifesto can also convert Tequila XML manifests to Vanguard manifests.

## Future Plans

Vanguard is still in early development and unstable.

A rough roadmap of planned development in order of priority is provided below.

* Expanded test coverage
* Makefiles and build pipelines
* Mirror ranking
* Associating files with profiles / selectively patching specific profiles *(breaks Tequila compatability)*

## Building

Vanguard can be built locally using the standard Rust toolchain and `cargo build`.

Binary distributions are planned once a stable version is ready.

## Contributing

Contributions are welcome, especially targeted at any of the Future Plans goals.

Please ensure that your code conforms to Rust style guidelines (`cargo fmt`) and that existing tests are passing (`cargo test`).

Pull requests without test coverage are discourage but will be considered, given the incomplete and unstable nature of the project.