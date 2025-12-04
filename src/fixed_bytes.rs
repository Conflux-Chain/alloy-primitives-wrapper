macro_rules! impl_bytes_wrapper {
    ($wrapper:ident, $alloy_type:ty, $ethereum_type:ty) => {
        pub struct $wrapper($alloy_type);

        impl From<$alloy_type> for $wrapper {
            fn from(a: $alloy_type) -> Self {
                $wrapper(a)
            }
        }

        impl From<&$alloy_type> for $wrapper {
            fn from(a: &$alloy_type) -> Self {
                $wrapper(a.to_owned())
            }
        }

        impl From<$wrapper> for $alloy_type {
            fn from(w: $wrapper) -> $alloy_type {
                w.0
            }
        }

        impl From<$ethereum_type> for $wrapper {
            fn from(a: $ethereum_type) -> Self {
                $wrapper(<$alloy_type>::from_slice(a.as_bytes()))
            }
        }

        impl From<&$ethereum_type> for $wrapper {
            fn from(a: &$ethereum_type) -> Self {
                $wrapper(<$alloy_type>::from_slice(a.as_bytes()))
            }
        }

        impl From<$wrapper> for $ethereum_type {
            fn from(w: $wrapper) -> $ethereum_type {
                <$ethereum_type>::from_slice(w.0.as_slice())
            }
        }
    };
}

// alloy_primitives: B32, B64, B128, B256, B512
// ethereum_types: H32, H64, H128, H256, H512
impl_bytes_wrapper!(WB32, alloy_primitives::aliases::B32, ethereum_types::H32);
impl_bytes_wrapper!(WB64, alloy_primitives::B64, ethereum_types::H64);
impl_bytes_wrapper!(WB128, alloy_primitives::B128, ethereum_types::H128);
impl_bytes_wrapper!(WB160, alloy_primitives::aliases::B160, ethereum_types::H160);
impl_bytes_wrapper!(WB256, alloy_primitives::B256, ethereum_types::H256);
impl_bytes_wrapper!(WB512, alloy_primitives::B512, ethereum_types::H512);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_bytes_conversion {
        ($test_name:ident, $wrapper:ty, $alloy_type:ty, $ethereum_type:ty, $size:expr) => {
            #[test]
            fn $test_name() {
                let bytes = [0x42u8; $size];
                let a = <$alloy_type>::from_slice(&bytes);
                let b: $wrapper = a.into();
                let c: $ethereum_type = b.into();
                let d: $alloy_type = <$wrapper>::from(c).into();
                assert_eq!(a, d);
            }
        };
    }

    test_bytes_conversion!(
        test_b64_conversion,
        WB64,
        alloy_primitives::B64,
        ethereum_types::H64,
        8
    );
    test_bytes_conversion!(
        test_b128_conversion,
        WB128,
        alloy_primitives::B128,
        ethereum_types::H128,
        16
    );
    test_bytes_conversion!(
        test_b256_conversion,
        WB256,
        alloy_primitives::B256,
        ethereum_types::H256,
        32
    );
    test_bytes_conversion!(
        test_b512_conversion,
        WB512,
        alloy_primitives::B512,
        ethereum_types::H512,
        64
    );
}
