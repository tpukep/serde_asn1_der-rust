use crate::{ Result, SerdeAsn1DerError };


/// A deserializer for the `Null` type
pub struct Null;
impl Null {
	/// The DER tag for the `Null` type
	pub const TAG: u8 = 0x05;
	
	/// Deserializes `Null` from `data`
	pub fn deserialize(data: &[u8]) -> Result<()> {
		Ok(match data.is_empty() {
			true => (),
			false => Err(SerdeAsn1DerError::InvalidData)?
		})
	}
}