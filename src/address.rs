use alloy_primitives::{Address, hex};
use std::{
    borrow::Borrow,
    fmt,
    ops::{Deref, DerefMut},
};

pub struct WAddress(Address);

impl Deref for WAddress {
    type Target = Address;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WAddress {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for WAddress {
    #[inline]
    fn default() -> Self {
        let a = Address::ZERO;
        WAddress(a)
    }
}

impl fmt::Debug for WAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::Display for WAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl AsRef<[u8]> for WAddress {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Borrow<[u8]> for WAddress {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl fmt::LowerHex for WAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&hex::encode_prefixed(self.as_ref()))
    }
}

impl fmt::UpperHex for WAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&hex::encode_upper_prefixed(self.as_ref()))
    }
}

impl From<Address> for WAddress {
    fn from(a: Address) -> Self {
        WAddress(a)
    }
}

impl From<&Address> for WAddress {
    fn from(a: &Address) -> Self {
        WAddress(a.to_owned())
    }
}

impl Into<Address> for WAddress {
    fn into(self) -> Address {
        self.0
    }
}

impl From<ethereum_types::Address> for WAddress {
    fn from(a: ethereum_types::Address) -> Self {
        WAddress(Address::from_slice(a.as_bytes()))
    }
}

impl From<&ethereum_types::Address> for WAddress {
    fn from(a: &ethereum_types::Address) -> Self {
        WAddress(Address::from_slice(a.as_bytes()))
    }
}

impl Into<ethereum_types::Address> for WAddress {
    fn into(self) -> ethereum_types::Address {
        ethereum_types::Address::from_slice(self.0.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, WAddress};

    #[test]
    fn test_deref() {
        let a = Address::ZERO;

        let b = WAddress(a);

        assert_eq!(a, *b);
    }
}
