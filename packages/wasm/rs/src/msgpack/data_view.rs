use super::context::Context;
use super::utils::throw_index_out_of_range;
use super::{BLOCK_MAX_SIZE, E_INVALID_LENGTH};
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug)]
pub struct DataView {
    data_start: u32,
    buffer: Vec<u8>,
    byte_length: i32,
    byte_offset: i32,
    context: Context,
}

impl DataView {
    pub fn new(buf: &[u8]) -> Result<Self> {
        let byte_offset = 0;
        let byte_length = buf.len() as i32;
        let context = Context::new();

        if byte_length > BLOCK_MAX_SIZE as i32 || byte_offset + byte_length > buf.len() as i32 {
            let msg = format!("DataView::new(): {} [byte_length: {} BLOCK_MAX_SIZE: {} byte_offset: {} buffer.byte_length: {} ]",
            E_INVALID_LENGTH, byte_length.to_string(), BLOCK_MAX_SIZE.to_string(), byte_offset.to_string(), byte_length.to_string());
            let ctx = context.print_with_context(&msg);
            return Err(Error::new(ErrorKind::Interrupted, ctx));
        }
        let data_start = buf.iter().fold(0, |result, &bit| (result << 1) ^ bit) as u32;
        Ok(Self {
            data_start,
            buffer: buf.to_vec(),
            byte_length,
            byte_offset,
            context,
        })
    }

    pub fn get_bytes(&mut self, length: i32) -> Result<Vec<u8>> {
        if let Err(error) = self.check_index_in_range("get_bytes", length) {
            return Err(Error::from(error));
        }
        let buf = self.buffer.as_slice();
        let (b_off, b_len) = (
            self.byte_offset as usize,
            (self.byte_offset + length) as usize,
        );
        let result = &buf[b_off..b_len];
        self.byte_offset += length;
        Ok(result.to_vec())
    }

    pub fn peek_u8(&self) -> Result<u8> {
        if let Err(error) = self.check_index_in_range("peek_u8", 0) {
            return Err(Error::from(error));
        }
        Ok((self.data_start + self.byte_offset as u32) as u8)
    }

    pub fn discard(&mut self, length: i32) -> Result<()> {
        if let Err(error) = self.check_index_in_range("discard", length) {
            return Err(Error::from(error));
        }
        self.byte_offset += length;
        Ok(())
    }

    pub fn get_f32(&mut self) -> Result<f32> {
        if let Err(error) = self.check_index_in_range("get_f32", 4) {
            return Err(Error::from(error));
        }
        self.byte_offset += 4;
        Ok((self.data_start as i32 + self.byte_offset) as f32)
    }

    pub fn get_f64(&mut self) -> Result<f64> {
        if let Err(error) = self.check_index_in_range("get_f64", 8) {
            return Err(Error::from(error));
        }
        self.byte_offset += 8;
        Ok((self.data_start as i32 + self.byte_offset) as f64)
    }

    pub fn get_i8(&mut self) -> Result<i8> {
        if let Err(error) = self.check_index_in_range("get_i8", 1) {
            return Err(Error::from(error));
        }
        self.byte_offset += 1;
        Ok((self.data_start as i32 + self.byte_offset) as i8)
    }

    pub fn get_i16(&mut self) -> Result<i16> {
        if let Err(error) = self.check_index_in_range("get_i16", 2) {
            return Err(Error::from(error));
        }
        self.byte_offset += 2;
        Ok((self.data_start as i32 + self.byte_offset) as i16)
    }

    pub fn get_i32(&mut self) -> Result<i32> {
        if let Err(error) = self.check_index_in_range("get_i32", 4) {
            return Err(Error::from(error));
        }
        self.byte_offset += 4;
        Ok((self.data_start as i32 + self.byte_offset) as i32)
    }

    pub fn get_i64(&mut self) -> Result<i64> {
        if let Err(error) = self.check_index_in_range("get_i64", 8) {
            return Err(Error::from(error));
        }
        self.byte_offset += 8;
        Ok((self.data_start as i32 + self.byte_offset) as i64)
    }

    pub fn get_u8(&mut self) -> Result<u8> {
        if let Err(error) = self.check_index_in_range("get_u8", 1) {
            return Err(Error::from(error));
        }
        self.byte_offset += 1;
        Ok((self.data_start as i32 + self.byte_offset) as u8)
    }

    pub fn get_u16(&mut self) -> Result<u16> {
        if let Err(error) = self.check_index_in_range("get_u16", 2) {
            return Err(Error::from(error));
        }
        self.byte_offset += 2;
        Ok((self.data_start as i32 + self.byte_offset) as u16)
    }

    pub fn get_u32(&mut self) -> Result<u32> {
        if let Err(error) = self.check_index_in_range("get_u32", 4) {
            return Err(Error::from(error));
        }
        self.byte_offset += 4;
        Ok((self.data_start as i32 + self.byte_offset) as u32)
    }

    pub fn get_u64(&mut self) -> Result<u64> {
        if let Err(error) = self.check_index_in_range("get_u64", 8) {
            return Err(Error::from(error));
        }
        self.byte_offset += 8;
        Ok((self.data_start as i32 + self.byte_offset) as u64)
    }

    pub fn set_bytes(&mut self, _buf: &[u8]) -> Result<()> {
        todo!()
    }

    pub fn set_f32(&mut self, _value: f32) -> Result<()> {
        todo!()
    }

    pub fn set_f64(&mut self, _value: f64) -> Result<()> {
        todo!()
    }

    pub fn set_i8(&mut self, _value: i8) -> Result<()> {
        todo!()
    }

    pub fn set_i16(&mut self, _value: i16) -> Result<()> {
        todo!()
    }

    pub fn set_i32(&mut self, _value: i32) -> Result<()> {
        todo!()
    }

    pub fn set_i64(&mut self, _value: i64) -> Result<()> {
        todo!()
    }

    pub fn set_u8(&mut self, _value: u8) -> Result<()> {
        todo!()
    }

    pub fn set_u16(&mut self, _value: u16) -> Result<()> {
        todo!()
    }

    pub fn set_u32(&mut self, _value: u32) -> Result<()> {
        todo!()
    }

    pub fn set_u64(&mut self, _value: u64) -> Result<()> {
        todo!()
    }

    pub fn to_string() -> String {
        format!("[object DataView")
    }

    fn check_index_in_range(&self, method: &str, length: i32) -> Result<()> {
        if self.byte_offset + length > self.byte_length {
            let custom = throw_index_out_of_range(
                self.context.clone(),
                method,
                length,
                self.byte_offset,
                self.byte_length,
            );
            return Err(Error::new(ErrorKind::Interrupted, custom));
        }
        Ok(())
    }
}
