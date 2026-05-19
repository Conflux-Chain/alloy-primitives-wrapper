//! Example: Converting between `alloy_primitives::U256` and `ethereum_types::U256` using `WU256`
//!
//! Below are common usage patterns demonstrating how to use `WU256` as an intermediary type
//! for bidirectional conversion between two U256 implementations.
//!
//! The approach: implement `From`/`Into` for `WU256` from both U256 types, enabling bridged
//! conversions through `WU256`.
//!
//! Simple example: convert from `alloy_primitives::U256` to `ethereum_types::U256` and back.
//!
//! ```rust
//! use alloy_primitives::U256 as AlloyU256;
//! use ethereum_types::U256 as EthU256;
//! use alloy_primitives_wrapper::WU256;
//!
//! let a = AlloyU256::try_from(42u64).unwrap();
//!
//! // Bridge through WU256: AlloyU256 -> WU256 -> EthU256
//! let eth: EthU256 = WU256::from(a).into();
//!
//! // Reverse: EthU256 -> WU256 -> AlloyU256
//! let e: EthU256 = EthU256::from(100u64);
//! let alloy: AlloyU256 = WU256::from(e).into();
//!
//! // Verify round-trip equality
//! let roundtrip: AlloyU256 = WU256::from(EthU256::from(42u64)).into();
//! assert_eq!(a, roundtrip);
//! ```
//!
//! This demonstrates the common pattern of using `WU256` as a bridge for conversions
//! between `alloy_primitives::U256` and `ethereum_types::U256`.
mod address;
mod fixed_bytes;
mod uint;

pub use address::WAddress;
pub use fixed_bytes::*;
pub use uint::*;
