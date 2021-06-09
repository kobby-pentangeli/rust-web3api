use crate::{Result, Write};
use num_bigint::BigInt;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct WriteSizer {
    length: i32,
}

impl WriteSizer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { length: 0 }
    }
}

impl Write for WriteSizer {
    fn write_nil(&mut self) {
        self.length += 1;
    }

    fn write_bool(&mut self, _value: bool) {
        self.length += 1;
    }

    fn write_i8(&mut self, value: i8) {
        self.write_i64(value as i64);
    }

    fn write_i16(&mut self, value: i16) {
        self.write_i64(value as i64);
    }

    fn write_i32(&mut self, value: i32) {
        self.write_i64(value as i64);
    }

    fn write_i64(&mut self, value: i64) {
        if value >= -(1 << 5) && value < 1 << 7 {
            self.length += 1;
        } else if value < 1 << 7 && value >= -(1 << 7) {
            self.length += 2;
        } else if value < 1 << 15 && value >= -(1 << 15) {
            self.length += 3;
        } else if value < 1 << 31 && value >= -(1 << 31) {
            self.length += 5;
        } else {
            self.length += 9;
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.write_u64(value as u64);
    }

    fn write_u16(&mut self, value: u16) {
        self.write_u64(value as u64);
    }

    fn write_u32(&mut self, value: u32) {
        self.write_u64(value as u64);
    }

    fn write_u64(&mut self, value: u64) {
        if value < 1 << 7 {
            self.length += 1;
        } else if value < 1 << 8 {
            self.length += 2;
        } else if value < 1 << 16 {
            self.length += 3;
        } else if value < 1 << 32 {
            self.length += 5;
        } else {
            self.length += 9;
        }
    }

    fn write_f32(&mut self, _value: f32) {
        self.length += 5;
    }

    fn write_f64(&mut self, _value: f64) {
        self.length += 9;
    }

    fn write_string_length(&mut self, length: u32) {
        if length < 32 {
            self.length += 1;
        } else if length <= u8::MAX as u32 {
            self.length += 2;
        } else if length <= u16::MAX as u32 {
            self.length += 3;
        } else {
            self.length += 5;
        }
    }

    fn write_string(&mut self, value: String) {
        self.write_string_length(value.len() as u32);
        self.length += value.len() as i32;
    }

    fn write_bytes_length(&mut self, length: u32) {
        if length <= u8::MAX as u32 {
            self.length += 2;
        } else if length <= u16::MAX as u32 {
            self.length += 3;
        } else {
            self.length += 5;
        }
    }

    fn write_bytes(&mut self, value: &[u8]) -> Result {
        if value.len() == 0 {
            self.length += 1;
            return Ok(());
        }
        self.write_bytes_length(value.len() as u32);
        self.length += value.len() as i32;
        Ok(())
    }

    fn write_bigint(&mut self, value: BigInt) {
        let val_str = value.to_string();
        self.write_string(val_str);
    }

    fn write_array_length(&mut self, length: u32) {
        if length < 16 {
            self.length += 1;
        } else if length <= u16::MAX as u32 {
            self.length += 3;
        } else {
            self.length += 5;
        }
    }

    fn write_array<T, F>(&mut self, _a: &[T], _func: F)
    where
        F: FnMut(u32),
    {
        todo!()
    }

    fn write_map_length(&mut self, length: u32) {
        if length < 16 {
            self.length += 1;
        } else if length <= u16::MAX as u32 {
            self.length += 3;
        } else {
            self.length += 5;
        }
    }

    fn write_map<F, K, V, W>(&mut self, _map: HashMap<K, V>, _key_fn: F, _value_fn: W)
    where
        F: FnMut(&K),
        W: FnMut(&V),
    {
        todo!()
    }

    fn write_nullable_bool(&mut self, value: Option<bool>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_bool(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_i8(&mut self, value: Option<i8>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_i8(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_i16(&mut self, value: Option<i16>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_i16(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_i32(&mut self, value: Option<i32>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_i32(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_i64(&mut self, value: Option<i64>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_i64(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_u8(&mut self, value: Option<u8>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_u8(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_u16(&mut self, value: Option<u16>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_u16(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_u32(&mut self, value: Option<u32>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_u32(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_u64(&mut self, value: Option<u64>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_u64(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_f32(&mut self, value: Option<f32>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_f32(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_f64(&mut self, value: Option<f64>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_f64(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_string(&mut self, value: Option<String>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_string(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_bytes(&mut self, value: Option<Vec<u8>>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        let _ = self.write_bytes(&value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_bigint(&mut self, value: Option<BigInt>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_bigint(value.unwrap_or_default());
        Ok(())
    }

    fn write_nullable_array<T, F>(&mut self, _a: Option<Vec<T>>, _func: F) -> Result
    where
        F: FnMut(T),
    {
        todo!()
    }

    fn write_nullable_map<F, K, V, W>(
        &mut self,
        _map: Option<HashMap<K, V>>,
        _key_fn: F,
        _value_fn: W,
    ) -> Result
    where
        F: FnMut(&K),
        W: FnMut(&V),
    {
        todo!()
    }
}
