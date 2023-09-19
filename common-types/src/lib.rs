#![cfg_attr(not(feature = "std"), no_std)]

//! # This library implements an U256Wrapper type which behaves similarly to the uint256 type in solidity.
//! # ATTENTION: It shouldn't be assumed that the types `uint256` and `U256Wrapper` are identical but rather the purpose of the U256Wrapper is to simplify 
//! # the integration of backends that already deal with values of the type format uint256. 
//! # The U256Wrapper type is meant to be used for token amounts and hence behaves according to the 
//! # CIS-2 specifications on Concordium.
use concordium_std::*;
use core::fmt::Debug;

#[cfg(feature = "u256_amount")]
pub use primitive_types::U256;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct U256Wrapper(pub U256);

/// Uses the ULeb128 encoding with up to 37 bytes for the encoding as
/// according to CIS-2 specification.
impl schema::SchemaType for U256Wrapper {
    fn get_type() -> schema::Type {
        schema::Type::ULeb128(37)
    }
}

impl Serial for U256Wrapper {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        let mut value = self.0;
        loop {
            let mut byte = (value.low_u32() as u8) & 0b0111_1111;
            value >>= 7;
            if value != U256::zero() {
                byte |= 0b1000_0000;
            }
            out.write_u8(byte)?;

            if value.is_zero() {
                return Ok(());
            }
        }
    }
}

impl Deserial for U256Wrapper {
    fn deserial<R: Read>(source: &mut R) -> ParseResult<Self> {
        let mut result: U256 = U256::zero();
        for i in 0..36 {
            let byte = source.read_u8()?;
            let value_byte = <U256>::from(byte & 0b0111_1111);
            result = result
                .checked_add(value_byte << (i * 7))
                .ok_or(ParseError {})?;
            if byte & 0b1000_0000 == 0 {
                return Ok(U256Wrapper(result));
            }
        }
        let byte = source.read_u8()?;
        let value_byte = byte & 0b0111_1111;
        if value_byte & 0b1111_0000 != 0 {
            Err(ParseError {})
        } else {
            let value_byte = <U256>::from(value_byte);
            result = result
                .checked_add(value_byte << (36 * 7))
                .ok_or(ParseError {})?;
            if byte & 0b1000_0000 == 0 {
                Ok(U256Wrapper(result))
            } else {
                Err(ParseError {})
            }
        }
    }
}

impl ops::Mul<U256Wrapper> for U256Wrapper {
    type Output = Self;

    fn mul(self, rhs: U256Wrapper) -> Self::Output {
        U256Wrapper(self.0 * rhs.0)
    }
}

