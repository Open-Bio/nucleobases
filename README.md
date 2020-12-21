# Nucleobases

![Crates.io](https://img.shields.io/crates/d/nucleobases) ![Crates.io](https://img.shields.io/crates/l/nucleobases) ![Crates.io](https://img.shields.io/crates/v/nucleobases)

A simple, low-level brick crate for managing nucleobases as data in code.

Nucleobases is a simple rust library that provides a universal data structure for handling nucleobases in rust code.

In addition to the Nucleobase enum the library provides a variety of helper methods to convert from text code to nucleobase, and to get key characteristics for each Nucleobase.

Currently, helper methods exist for determining whether or not the base is a(n)
- Purine
- Pyrimidine
- Ketone
- Amine
- Ribonucleotide base
- Deoxyribonucleotide base

Nucleobases is purposefully barebones to provide very little overhead in projects that want to use it, but allow for a standard nucleobase data type for inter-library uses.

As of version 1.0.0 the api for Nucleobases has been stabilized.

### License

Nucleobases can be licensed under either the Apache License Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0) or the MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT) at your option.

### Contribution

Unless explicitly stated otherwise, by intentionally submitting a contribution to the "nucleobases" repository for inclusion you implicitly cede all copyrights to your contribution to The Open Bio Project, as defined by the Apache License Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0). Additionally, your contribution will be dual licensed, as outlined above, without any additional terms or conditions.
