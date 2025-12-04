# alloy-primitives-converter

This library defines wrapper types for alloy-primitives base types to provide convenient conversions between alloy-primitives and ethereum-types(0.9.0).

```rust
use alloy_primitives_wrapper::WU256;
use alloy_primitives::U256;
use ethereum_types as eth_types;

let a = U256::from(100);
let b: eth_types::U256 = WU256::from(a).into();

let c = eth_types::U256::from(100);
let d: U256 = WU256::from(c).into();
```