# rust-benchmark-example

Simple example on how to do benchmarks in Rust using the "criterion" library.

## How to use

Insteall the cargo-criterion command:

```bash
cargo install cargo-criterion
```

Run benchmarks:

```bash
cargo criterion
```

For this particular example, the output should look like:

```plain
running 1 test
test tests::verifying_secp256k1 ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

Gnuplot not found, using plotters backend
secp256k1_verify/overhead included
                        time:   [154.61 µs 155.09 µs 155.63 µs]
secp256k1_verify/overhead excluded
                        time:   [72.455 µs 72.599 µs 72.767 µs]
```
