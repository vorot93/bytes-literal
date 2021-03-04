#[doc(hidden)]
pub extern crate lifetimed_bytes;

#[doc(hidden)]
pub extern crate hex_literal;

#[macro_export]
macro_rules! bytes {
    ( $x:expr ) => {
        $crate::lifetimed_bytes::Bytes::from(&$crate::hex_literal::hex!($x) as &[u8])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let a = bytes!["0080ff"];
        let b = vec![0_u8, 128, 255];
        assert_eq!(a, b);
        assert_eq!(a, lifetimed_bytes::Bytes::from(b.as_slice()));
    }
}
