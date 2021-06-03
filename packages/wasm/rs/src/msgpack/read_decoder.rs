use super::data_view::DataView;
use super::format::Format;
use super::read::Read;
use super::E_INVALID_LENGTH;
use num_bigint::BigInt;

use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::str::FromStr;

struct ReadDecoder {
    view: DataView,
}

impl ReadDecoder {
    #[allow(dead_code)]
    pub fn new(buffer: &[u8]) -> Self {
        Self {
            view: DataView::new(buffer).unwrap(),
        }
    }

    pub fn is_next_nil(&mut self) -> bool {
        let format = self.view.peek_u8().unwrap();
        if format == Format::NIL {
            let _ = self.view.discard(1);
            return true;
        }
        false
    }

    #[allow(dead_code)]
    pub fn is_next_string(&mut self) -> bool {
        let format = self.view.peek_u8().unwrap();
        return Format::is_fixed_string(format)
            || format == Format::STR8
            || format == Format::STR16
            || format == Format::STR32;
    }

    #[allow(dead_code)]
    fn skip(&mut self) {
        // get_size handles discarding `msgpack header` info
        let mut num_of_objects_to_discard = self.get_size().unwrap();
        while num_of_objects_to_discard > 0 {
            let _ = self.get_size().unwrap(); // discard next object
            num_of_objects_to_discard -= 1;
        }
    }

    fn get_size(&mut self) -> Result<i32> {
        let lead_byte = self.view.get_u8()?; // will discard one
        let mut objects_to_discard: i32 = 0;
        // handle for fixed values
        if Format::is_negative_fixed_int(lead_byte) {
            // noop, will just discard the leadbyte
        } else if Format::is_fixed_int(lead_byte) {
            // noop, will just discard the leadbyte
        } else if Format::is_fixed_string(lead_byte) {
            let str_len = lead_byte & 0x1f;
            let _ = self.view.discard(str_len as i32)?;
        } else if Format::is_fixed_array(lead_byte) {
            objects_to_discard = (lead_byte & Format::FOUR_LEAST_SIG_BITS_IN_BYTE) as i32;
        } else if Format::is_fixed_map(lead_byte) {
            objects_to_discard = 2 * (lead_byte & Format::FOUR_LEAST_SIG_BITS_IN_BYTE) as i32;
        } else {
            match lead_byte {
                Format::NIL => {}
                Format::TRUE => {}
                Format::FALSE => {}
                Format::BIN8 => {
                    let length = self.view.get_u8()?;
                    let _ = self.view.discard(length as i32)?;
                }
                Format::BIN16 => {
                    let length = self.view.get_u16()?;
                    let _ = self.view.discard(length as i32)?;
                }
                Format::BIN32 => {
                    let length = self.view.get_u32()?;
                    let _ = self.view.discard(length as i32)?;
                }
                Format::FLOAT32 => {
                    let _ = self.view.discard(4)?;
                }
                Format::FLOAT64 => {
                    let _ = self.view.discard(8)?;
                }
                Format::UINT8 => {
                    let _ = self.view.discard(1)?;
                }
                Format::UINT16 => {
                    let _ = self.view.discard(2)?;
                }
                Format::UINT32 => {
                    let _ = self.view.discard(4)?;
                }
                Format::UINT64 => {
                    let _ = self.view.discard(8)?;
                }
                Format::INT8 => {
                    let _ = self.view.discard(1)?;
                }
                Format::INT16 => {
                    let _ = self.view.discard(2)?;
                }
                Format::INT32 => {
                    let _ = self.view.discard(4)?;
                }
                Format::INT64 => {
                    let _ = self.view.discard(8)?;
                }
                Format::FIXEXT1 => {
                    let _ = self.view.discard(2)?;
                }
                Format::FIXEXT2 => {
                    let _ = self.view.discard(3)?;
                }
                Format::FIXEXT4 => {
                    let _ = self.view.discard(5)?;
                }
                Format::FIXEXT8 => {
                    let _ = self.view.discard(9)?;
                }
                Format::FIXEXT16 => {
                    let _ = self.view.discard(17)?;
                }
                Format::STR8 => {
                    let length = self.view.get_u8()?;
                    let _ = self.view.discard(length as i32)?;
                }
                Format::STR16 => {
                    let length = self.view.get_u16()?;
                    let _ = self.view.discard(length as i32)?;
                }
                Format::STR32 => {
                    let length = self.view.get_u32()?;
                    let _ = self.view.discard(length as i32)?;
                }
                Format::ARRAY16 => {
                    objects_to_discard = self.view.get_u16()? as i32;
                }
                Format::ARRAY32 => {
                    objects_to_discard = self.view.get_u32()? as i32;
                }
                Format::MAP16 => {
                    objects_to_discard = 2 * (self.view.get_u16()? as i32);
                }
                Format::MAP32 => {
                    objects_to_discard = 2 * (self.view.get_u32()? as i32);
                }
                _ => {
                    let custom_error = format!(
                        "ivalid prefix, bad encoding for val: {}",
                        lead_byte.to_string()
                    );
                    return Err(Error::new(ErrorKind::Other, custom_error));
                }
            }
        }

        Ok(objects_to_discard)
    }
}

impl Read for ReadDecoder {
    fn read_bool(&mut self) -> Result<bool> {
        if let Ok(value) = self.view.get_u8() {
            if value == Format::TRUE {
                return Ok(true);
            } else if value == Format::FALSE {
                return Ok(false);
            }
        }
        Err(Error::new(ErrorKind::Other, "bad value for bool"))
    }

    fn read_i8(&mut self) -> Result<i8> {
        let value = self.read_i64()?;
        if (value <= i8::MAX as i64) && (value >= i8::MIN as i64) {
            return Ok(value as i8);
        }
        let custom_error = format!("integer overflow: value = {}; bits = 8", value.to_string());
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_i16(&mut self) -> Result<i16> {
        let value = self.read_i64()?;
        if (value <= i16::MAX as i64) && (value >= i16::MIN as i64) {
            return Ok(value as i16);
        }
        let custom_error = format!("integer overflow: value = {}; bits = 16", value.to_string());
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let value = self.read_i64()?;
        if (value <= i32::MAX as i64) && (value >= i32::MIN as i64) {
            return Ok(value as i32);
        }
        let custom_error = format!("integer overflow: value = {}; bits = 32", value.to_string());
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let prefix = self.view.get_u8()?;
        if Format::is_fixed_int(prefix) {
            return Ok(prefix as i64);
        }
        if Format::is_negative_fixed_int(prefix) {
            return Ok((prefix as i8) as i64);
        }
        match prefix {
            Format::INT8 => Ok(self.view.get_i8()? as i64),
            Format::INT16 => Ok(self.view.get_i16()? as i64),
            Format::INT32 => Ok(self.view.get_i32()? as i64),
            Format::INT64 => Ok(self.view.get_i64()?),
            _ => {
                let custom_error = format!("bad prefix for int: {}", prefix.to_string());
                Err(Error::new(ErrorKind::Other, custom_error))
            }
        }
    }

    fn read_u8(&mut self) -> Result<u8> {
        let value = self.read_u64()?;
        if (value <= u8::MAX as u64) && (value >= u8::MIN as u64) {
            return Ok(value as u8);
        }
        let custom_error = format!(
            "unsigned integer overflow: value = {}; bits = 8",
            value.to_string()
        );
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_u16(&mut self) -> Result<u16> {
        let value = self.read_u64()?;
        if (value <= u16::MAX as u64) && (value >= u16::MIN as u64) {
            return Ok(value as u16);
        }
        let custom_error = format!(
            "unsigned integer overflow: value = {}; bits = 16",
            value.to_string()
        );
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let value = self.read_u64()?;
        if (value <= u32::MAX as u64) && (value >= u32::MIN as u64) {
            return Ok(value as u32);
        }
        let custom_error = format!(
            "unsigned integer overflow: value = {}; bits = 32",
            value.to_string()
        );
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let prefix = self.view.get_u8()?;
        if Format::is_fixed_int(prefix) {
            return Ok(prefix as u64);
        } else if Format::is_negative_fixed_int(prefix) {
            let custom_error = format!(
                "unsigned integer cannot be negative: prefix = {}",
                prefix.to_string()
            );
            return Err(Error::new(ErrorKind::Other, custom_error));
        }
        match prefix {
            Format::UINT8 => Ok(self.view.get_u8()? as u64),
            Format::UINT16 => Ok(self.view.get_u16()? as u64),
            Format::UINT32 => Ok(self.view.get_u32()? as u64),
            Format::UINT64 => Ok(self.view.get_u64()?),
            _ => {
                let custom_error = format!("bad prefix for unsigned int: {}", prefix.to_string());
                Err(Error::new(ErrorKind::Other, custom_error))
            }
        }
    }

    fn read_f32(&mut self) -> Result<f32> {
        let prefix = self.view.get_u8()?;
        if Format::is_float_32(prefix) {
            return Ok(self.view.get_f32()?);
        }
        let custom_error = format!("bad prefix for float 32: {}", prefix.to_string());
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_f64(&mut self) -> Result<f64> {
        let prefix = self.view.get_u8()?;
        if Format::is_float_64(prefix) {
            return Ok(self.view.get_f64()?);
        }
        let custom_error = format!("bad prefix for float 64: {}", prefix.to_string());
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_string_length(&mut self) -> Result<u32> {
        let lead_byte = self.view.get_u8()?;
        if Format::is_fixed_string(lead_byte) {
            return Ok((lead_byte & 0x1f) as u32);
        }
        if Format::is_fixed_array(lead_byte) {
            return Ok((lead_byte & Format::FOUR_LEAST_SIG_BITS_IN_BYTE) as u32);
        }
        match lead_byte {
            Format::STR8 => Ok(self.view.get_u8()? as u32),
            Format::STR16 => Ok(self.view.get_u16()? as u32),
            Format::STR32 => Ok(self.view.get_u32()?),
            _ => {
                let custom_error = format!("{} {}", E_INVALID_LENGTH, lead_byte.to_string());
                Err(Error::new(ErrorKind::Other, custom_error))
            }
        }
    }

    fn read_string(&mut self) -> Result<String> {
        let str_len = self.read_string_length()?;
        let str_bytes = self.view.get_bytes(str_len as i32)?;
        Ok(String::from_utf8(str_bytes).unwrap())
    }

    fn read_bytes_length(&mut self) -> Result<u32> {
        if self.is_next_nil() {
            return Ok(0);
        }
        let lead_byte = self.view.get_u8()?;
        if Format::is_fixed_string(lead_byte) {
            return Ok((lead_byte & 0x1f) as u32);
        }
        if Format::is_fixed_array(lead_byte) {
            return Ok((lead_byte & Format::FOUR_LEAST_SIG_BITS_IN_BYTE) as u32);
        }
        match lead_byte {
            Format::STR8 => Ok(self.view.get_u8()? as u32),
            Format::STR16 => Ok(self.view.get_u16()? as u32),
            Format::STR32 => Ok(self.view.get_u32()?),
            _ => {
                let custom_error = format!("{} {}", E_INVALID_LENGTH, lead_byte.to_string());
                Err(Error::new(ErrorKind::Other, custom_error))
            }
        }
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>> {
        let array_length = self.read_bytes_length()?;
        Ok(self.view.get_bytes(array_length as i32)?)
    }

    fn read_bigint(&mut self) -> Result<BigInt> {
        let s = self.read_string()?;
        Ok(BigInt::from_str(&s).unwrap())
    }

    fn read_array_length(&mut self) -> Result<u32> {
        let lead_byte = self.view.get_u8()?;
        if Format::is_fixed_array(lead_byte) {
            return Ok((lead_byte & Format::FOUR_LEAST_SIG_BITS_IN_BYTE) as u32);
        } else if lead_byte == Format::ARRAY16 {
            let r = self.view.get_u16()?;
            return Ok(r as u32);
        } else if lead_byte == Format::ARRAY32 {
            return Ok(self.view.get_u32()?);
        } else if lead_byte == Format::NIL {
            return Ok(0);
        }
        let custom_error = format!("{} {}", E_INVALID_LENGTH, lead_byte.to_string());
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_array<T, F>(&mut self, _func: F) -> Result<Vec<T>>
    where
        F: FnMut(u32),
    {
        todo!()
    }

    fn read_map_length(&mut self) -> Result<u32> {
        let lead_byte = self.view.get_u8()?;
        if Format::is_fixed_map(lead_byte) {
            return Ok((lead_byte & Format::FOUR_LEAST_SIG_BITS_IN_BYTE) as u32);
        } else if lead_byte == Format::MAP16 {
            return Ok((self.view.get_u16()?) as u32);
        } else if lead_byte == Format::MAP32 {
            return Ok(self.view.get_u32()?);
        }
        let custom_error = format!("{}", E_INVALID_LENGTH);
        Err(Error::new(ErrorKind::Other, custom_error))
    }

    fn read_map<F, K, V, W>(&mut self, _key_fn: F,_value_fnn: W) -> HashMap<K, V>
    where
        F: FnMut(&K),
        W: FnMut(&V),
    {
        todo!()
    }

    fn read_nullable_bool(&mut self) -> Option<bool> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_bool().unwrap())
    }

    fn read_nullable_i8(&mut self) -> Option<i8> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_i8().unwrap())
    }

    fn read_nullable_i16(&mut self) -> Option<i16> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_i16().unwrap())
    }

    fn read_nullable_i32(&mut self) -> Option<i32> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_i32().unwrap())
    }

    fn read_nullable_i64(&mut self) -> Option<i64> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_i64().unwrap())
    }

    fn read_nullable_u8(&mut self) -> Option<u8> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_u8().unwrap())
    }

    fn read_nullable_u16(&mut self) -> Option<u16> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_u16().unwrap())
    }

    fn read_nullable_u32(&mut self) -> Option<u32> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_u32().unwrap())
    }

    fn read_nullable_u64(&mut self) -> Option<u64> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_u64().unwrap())
    }

    fn read_nullable_f32(&mut self) -> Option<f32> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_f32().unwrap())
    }

    fn read_nullable_f64(&mut self) -> Option<f64> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_f64().unwrap())
    }

    fn read_nullable_string(&mut self) -> Option<String> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_string().unwrap())
    }

    fn read_nullable_bytes(&mut self) -> Option<Vec<u8>> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_bytes().unwrap())
    }

    fn read_nullable_bigint(&mut self) -> Option<BigInt> {
        if self.is_next_nil() {
            return None;
        }
        Some(self.read_bigint().unwrap())
    }

    fn read_nullable_array<T, F>(&mut self, _func: F) -> Option<Vec<T>>
    where
        F: FnMut(T),
    {
        todo!()
    }

    fn read_nullable_map<F, K, V, W>(&mut self, _key_fn: F,_value_fnn: W) -> Option<HashMap<K, V>>
    where
        F: FnMut(&K),
        W: FnMut(&V),
    {
        todo!()
    }
}
