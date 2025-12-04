macro_rules! impl_uint_wrapper {
    ($wrapper:ident, $alloy_type:ty, $ethereum_type:ty, $size:expr) => {
        pub struct $wrapper($alloy_type);

        impl From<$alloy_type> for $wrapper {
            fn from(a: $alloy_type) -> Self {
                $wrapper(a)
            }
        }

        impl From<$wrapper> for $alloy_type {
            fn from(w: $wrapper) -> $alloy_type {
                w.0
            }
        }

        impl From<&$alloy_type> for $wrapper {
            fn from(a: &$alloy_type) -> Self {
                $wrapper(a.to_owned())
            }
        }

        impl From<$ethereum_type> for $wrapper {
            fn from(a: $ethereum_type) -> Self {
                let mut raw: [u8; $size] = [0u8; $size];
                a.to_big_endian(&mut raw);
                $wrapper(<$alloy_type>::from_be_bytes(raw))
            }
        }

        impl From<&$ethereum_type> for $wrapper {
            fn from(a: &$ethereum_type) -> Self {
                let mut raw: [u8; $size] = [0u8; $size];
                a.to_big_endian(&mut raw);
                $wrapper(<$alloy_type>::from_be_bytes(raw))
            }
        }

        impl From<$wrapper> for $ethereum_type {
            fn from(w: $wrapper) -> $ethereum_type {
                let raw: [u8; $size] = w.0.to_be_bytes();
                <$ethereum_type>::from_big_endian(&raw)
            }
        }
    };
}

// ethereum-types 只有 U64, U128, U256, U512
impl_uint_wrapper!(WU64, alloy_primitives::U64, ethereum_types::U64, 8);
impl_uint_wrapper!(WU128, alloy_primitives::U128, ethereum_types::U128, 16);
impl_uint_wrapper!(WU256, alloy_primitives::U256, ethereum_types::U256, 32);
impl_uint_wrapper!(WU512, alloy_primitives::U512, ethereum_types::U512, 64);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_conversion {
        ($test_name:ident, $wrapper:ty, $alloy_type:ty, $ethereum_type:ty) => {
            #[test]
            fn $test_name() {
                let a = <$alloy_type>::from(0x1234u64);
                let b: $wrapper = a.into();
                let c: $ethereum_type = b.into();
                let d: $alloy_type = <$wrapper>::from(c).into();
                assert_eq!(a, d);
            }
        };
    }

    test_conversion!(
        test_u64_conversion,
        WU64,
        alloy_primitives::U64,
        ethereum_types::U64
    );
    test_conversion!(
        test_u128_conversion,
        WU128,
        alloy_primitives::U128,
        ethereum_types::U128
    );
    test_conversion!(
        test_u256_conversion,
        WU256,
        alloy_primitives::U256,
        ethereum_types::U256
    );
    test_conversion!(
        test_u512_conversion,
        WU512,
        alloy_primitives::U512,
        ethereum_types::U512
    );
}
