use crate::DecodeError;
#[cfg(feature = "std")]
use alloc::format;

/// Represent the wire type for protobuf encoding.
///
/// The integer value is equvilant with the encoded value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WireType {
    Varint = 0,
    SixtyFourBit = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    ThirtyTwoBit = 5,
}

impl TryFrom<u64> for WireType {
    type Error = DecodeError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::SixtyFourBit),
            2 => Ok(WireType::LengthDelimited),
            3 => Ok(WireType::StartGroup),
            4 => Ok(WireType::EndGroup),
            5 => Ok(WireType::ThirtyTwoBit),
            _ => Err(invalid_wire_type_value(value)),
        }
    }
}

// Out of line so the formatting isn't inlined into every field's decoder.
#[cold]
#[inline(never)]
fn invalid_wire_type_value(_value: u64) -> DecodeError {
    #[cfg(feature = "std")]
    return DecodeError::new(format!("invalid wire type value: {}", _value));
    // Without `std`, a static message keeps core::fmt's integer formatting out of the image.
    #[cfg(not(feature = "std"))]
    return DecodeError::new("invalid wire type value");
}

#[cold]
#[inline(never)]
fn wire_type_mismatch(_expected: WireType, _actual: WireType) -> DecodeError {
    #[cfg(feature = "std")]
    return DecodeError::new(format!(
        "invalid wire type: {:?} (expected {:?})",
        _actual, _expected
    ));
    #[cfg(not(feature = "std"))]
    return DecodeError::new("invalid wire type");
}

/// Checks that the expected wire type matches the actual wire type,
/// or returns an error result.
#[inline]
pub fn check_wire_type(expected: WireType, actual: WireType) -> Result<(), DecodeError> {
    if expected != actual {
        return Err(wire_type_mismatch(expected, actual));
    }
    Ok(())
}
