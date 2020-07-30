# Open DNA - Bases

Core rust library for managing nucleobases in code.

Open DNA - Bases (henceforth referred to as simply "Bases") is a simple rust library that provides a universal data structure for handling nucleobases in rust code.

In addition to the Nucleobase enum the library provides a variety of helper methods to convert from text code to nucleobase, and to get key characteristics for each Nucleobase.

Currently, helper methods exist for determining whether or not the base is a(n)
- Purine
- Pyrimidine
- Ketone
- Amine
- Ribonucleotide base
- Deoxyribonucleotide base

Bases is purposefully barebones to provide very little overhead in projects that want to use it, but allow for a standard nucleobase data type for inter-library uses.
