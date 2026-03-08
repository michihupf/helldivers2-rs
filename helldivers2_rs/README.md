# Helldivers 2 API wrapper in Rust
This is a wrapper around the [Helldivers 2 API](https://github.com/helldivers-2/api)
provided by the [Helldivers 2 Community](https://github.com/helldivers-2) community implemented 
in the Rust programming language.

All relevant calls are done through `HellApi`. Make sure to set client and contact information
as per API specification using `HellApi::init()`.

# A note on Documentation
The project significantly lacks extensive documentation as it is still in early development.
The first major version will include extensive documentation and code examples.
Basic documenation is included.

Early development will continue on branch `master` until the first major version. From there 
further development will be done on `next` and `dev` before being remerged to `master`.
The project is still usable as of now but the early minor versions will not neccessarily be backwards compatible.

Also, this is my first Open Source Rust project and my first API wrapper. I am open to suggestions, tips 
and pull requests for improving readability, code layout and overall performance.
