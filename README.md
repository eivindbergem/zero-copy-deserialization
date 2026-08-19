# Implementing zero-copy (de)serialization in Rust from scratch

This repo contains code from [the
presentation](https://ndctechtown.com/agenda/implementing-zero-copy-deserialization-in-rust-from-scratch-0d5x/0hjpcm372zj)
I did at NDC TechTown 2026.

## Commit history

The commits in this repo shows the evolution of the (de)serialization,
following the steps in the presentation.

## Tests

To test the serialization format across platform, the test script uses
[cross](https://github.com/cross-rs/cross).

To run tests:

```shell
cd tests
./run-tests.sh
```
