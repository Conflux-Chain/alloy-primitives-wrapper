//! 示例：使用 WU256 在 alloy_primitives::U256 与 ethereum_types::U256 之间相互转换
//!
//! 下面给出若干常见用法与模式，重点展示如何把 WU256 作为中间类型来实现两种 U256 类型之间的相互转换。
//! 假设 crate 中导出：crate::WU256、alloy_primitives::U256、ethereum_types::U256。
//!
//! 主要思路：为 WU256 提供从两端 U256 的 From/Into 支持，然后在需要时直接通过 WU256 做桥接转换。
//!
//! 简单示例：从 alloy_primitives::U256 转换到 ethereum_types::U256，并反向转换。
//!
//! ```rust
//! use alloy_primitives::U256 as AlloyU256;
//! use ethereum_types::U256 as EthU256;
//! use crate::WU256;
//!
//! // 假设两边都实现了从原始整数类型的转换（常见于 U256 实现）
//! let a: AlloyU256 = 42u64.into();
//!
//! // 通过 WU256 做桥接：AlloyU256 -> WU256 -> EthU256
//! let eth: EthU256 = WU256::from(a).into();
//!
//! // 反向：EthU256 -> WU256 -> AlloyU256
//! let e: EthU256 = EthU256::from(100u64);
//! let alloy: AlloyU256 = WU256::from(e).into();
//!
//! // 验证来回不变（示例断言）
//! assert_eq!(AlloyU256::from(u64::from(42u64)), AlloyU256::from(WU256::from(EthU256::from(42u64))));
//! ```
//!
//! 以上就是使用 WU256 作为桥接，实现在 alloy_primitives::U256 与 ethereum_types::U256 之间相互转换的常见用法示例。
mod address;
mod fixed_bytes;
mod uint;

pub use address::WAddress;
pub use fixed_bytes::*;
pub use uint::*;
