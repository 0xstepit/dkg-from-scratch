# DKG From Scratch

Implementation of the Distributed Key Generation (DKG) protocol for the BLS12-381 elliptic curve.

The DKG is build sequentially one component on top of the other:

- Shamir's secret sharing.
- Feldman's verifiable secret sharing.
- Pedersen's distributed key generation via Joint-Feldman.

## Test

```rust
cargo test -p dkg -- --show-output
```

## Todo

- [ ] : add threshold signature reconstruction.
- [ ] : add complain handling logic.
- [ ] : add key resharing.
- [ ] : add key refreshing.
- [ ] : create a real network.

## References

- Pedersen, T. P. (1991). "A Threshold Cryptosystem without a Trusted Party"
- Feldman, P. (1987). "A Practical Scheme for Non-interactive Verifiable Secret Sharing"
- Gennaro et al. (1999). "Secure Distributed Key Generation for Discrete-Log Based Cryptosystems"
