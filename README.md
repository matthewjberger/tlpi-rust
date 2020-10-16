# tlpi-rust

Working through the linux programming interface examples in Rust 

## Docker

Build:

`docker build -t tlpi .`

Run interactively (windows powershell):

`docker run -v ${pwd}:/code/tlpi-rust:rw --rm -it tlpi /bin/bash`

Alternatively just use vscode's remote development plugin

Build docs in container:

`cargo doc`

Then open on host:

`target/doc/tlpi_rust/index.html`
