use super::{BLOCK_MAX_SIZE, E_INDEX_OUT_OF_RANGE, E_INVALID_LENGTH};
use std::io::{Error, ErrorKind, Result};

pub struct DataView {
    data_start: u32,
    buffer: Vec<u8>,
    byte_length: i32,
    byte_offset: i32,
}

impl DataView {
    pub fn new(buffer: &[u8]) -> Result<Self> {
        let byte_offset = 0;
        let byte_length = buffer.len() as i32;

        if byte_length > BLOCK_MAX_SIZE as i32 || byte_offset + byte_length > buffer.len() as i32 {
            eprint!("{}", E_INVALID_LENGTH);
            return Err(Error::new(ErrorKind::Interrupted, E_INVALID_LENGTH));
        }
        let data_start = buffer.iter().fold(0, |result, &bit| (result << 1) ^ bit) as u32;
        Ok(Self {
            data_start,
            buffer: buffer.to_vec(),
            byte_length,
            byte_offset,
        })
    }

    pub fn get_bytes(&mut self, length: i32) -> Result<Vec<u8>> {
        if self.byte_offset + length > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
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

    pub fn set_bytes(&mut self, buf: &[u8]) -> Result<()> {
        if self.byte_offset + buf.len() as i32 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        self.buffer.append(&mut buf.to_vec()); // Really?
        self.byte_offset += self.byte_length;
        Ok(())
    }

    pub fn peek_u8(&self) -> Result<u8> {
        if self.byte_offset > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        Ok((self.data_start + self.byte_offset as u32) as u8)
    }

    pub fn discard(&mut self, length: i32) -> Result<()> {
        if self.byte_offset + length > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        self.byte_offset += length;
        Ok(())
    }

    pub fn get_f32(&mut self) -> Result<f32> {
        if self.byte_offset + 4 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as f32;
        self.byte_offset += 4;
        Ok(result)
    }

    pub fn get_f64(&mut self) -> Result<f64> {
        if self.byte_offset + 8 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as f64;
        self.byte_offset += 8;
        Ok(result)
    }

    pub fn get_i8(&mut self) -> Result<i8> {
        if self.byte_offset >= self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as i8;
        self.byte_offset += 1;
        Ok(result)
    }

    pub fn get_i16(&mut self) -> Result<i16> {
        if self.byte_offset + 2 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as i16;
        self.byte_offset += 2;
        Ok(result)
    }

    pub fn get_i32(&mut self) -> Result<i32> {
        if self.byte_offset + 4 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as i32;
        self.byte_offset += 4;
        Ok(result)
    }

    pub fn get_i64(&mut self) -> Result<i64> {
        if self.byte_offset + 8 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as i64;
        self.byte_offset += 8;
        Ok(result)
    }

    pub fn get_u8(&mut self) -> Result<u8> {
        if self.byte_offset >= self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as u8;
        self.byte_offset += 1;
        Ok(result)
    }

    pub fn get_u16(&mut self) -> Result<u16> {
        if self.byte_offset + 2 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as u16;
        self.byte_offset += 2;
        Ok(result)
    }

    pub fn get_u32(&mut self) -> Result<u32> {
        if self.byte_offset + 4 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as u32;
        self.byte_offset += 4;
        Ok(result)
    }

    pub fn get_u64(&mut self) -> Result<u64> {
        if self.byte_offset + 8 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }
        let result = (self.data_start as i32 + self.byte_offset) as u64;
        self.byte_offset += 8;
        Ok(result)
    }

    pub fn set_f32(&mut self, value: f32) -> Result<()> {
        if self.byte_offset + 4 > self.byte_length {
            eprint!("{}", E_INDEX_OUT_OF_RANGE);
            return Err(Error::new(ErrorKind::Interrupted, E_INDEX_OUT_OF_RANGE));
        }

        Ok(())
    }
}
