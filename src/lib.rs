#![no_std]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub extern crate bytes;

#[doc(hidden)]
pub extern crate hex_literal;

#[macro_export]
macro_rules! bytes {
    ( $x:expr ) => {
        $crate::bytes::Bytes::from_static(&$crate::hex_literal::hex!($x))
    };
}
