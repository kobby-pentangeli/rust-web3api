use crate::{Nullable, Result, Write};
use num_bigint::BigInt;
use std::collections::HashMap;

struct WriteSizer {
    length: i32,
}

impl WriteSizer {
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

    fn write_string(&mut self, value: &str) {
        use std::str::FromStr;
        let buf = String::from_str(value).unwrap();
        self.write_string_length(buf.len() as u32);
        self.length += buf.len() as i32;
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
        self.write_string(&val_str);
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

    fn write_array<T, F>(&mut self, a: &[T], func: F)
    where
        F: FnMut(u32),
    {
        self.write_array_length(a.len() as u32);
        for i in 0..a.len() {
            func(i as u32);
        }
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

    fn write_map<F, K, V, W>(&mut self, map: HashMap<K, V>, key_fn: F, value_fn: W)
    where
        F: FnMut(&K),
        W: FnMut(&V),
    {
        self.write_map_length(map.len() as u32);
        for (key, value) in map {
            key_fn(&key);
            value_fn(&value);
        }
    }

    fn write_nullable_bool(&mut self, value: Nullable<bool>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_bool(*value.get_value());
        Ok(())
    }

    fn write_nullable_i8(&mut self, value: Nullable<i8>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_i8(*value.get_value());
        Ok(())
    }

    fn write_nullable_i16(&mut self, value: Nullable<i16>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_i16(*value.get_value());
        Ok(())
    }

    fn write_nullable_i32(&mut self, value: Nullable<i32>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_i32(*value.get_value());
        Ok(())
    }

    fn write_nullable_i64(&mut self, value: Nullable<i64>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_i64(*value.get_value());
        Ok(())
    }

    fn write_nullable_u8(&mut self, value: Nullable<u8>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_u8(*value.get_value());
        Ok(())
    }

    fn write_nullable_u16(&mut self, value: Nullable<u16>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_u16(*value.get_value());
        Ok(())
    }

    fn write_nullable_u32(&mut self, value: Nullable<u32>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_u32(*value.get_value());
        Ok(())
    }

    fn write_nullable_u64(&mut self, value: Nullable<u64>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_u64(*value.get_value());
        Ok(())
    }

    fn write_nullable_f32(&mut self, value: Nullable<f32>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_f32(*value.get_value());
        Ok(())
    }

    fn write_nullable_f64(&mut self, value: Nullable<f64>) -> Result {
        if value.is_null() {
            self.write_nil();
            return Ok(());
        }
        self.write_f64(*value.get_value());
        Ok(())
    }

    fn write_nullable_string(&mut self, value: Option<&str>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_string(value.unwrap());
        Ok(())
    }

    fn write_nullable_bytes(&mut self, value: Option<Vec<u8>>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_bytes(&value.unwrap());
        Ok(())
    }

    fn write_nullable_bigint(&mut self, value: Option<BigInt>) -> Result {
        if value.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_bigint(value.unwrap());
        Ok(())
    }

    fn write_nullable_array<T, F>(&mut self, a: Option<Vec<T>>, func: F) -> Result
    where
        F: FnMut(T),
    {
        if a.is_none() {
            self.write_nil();
            return Ok(());
        }
        self.write_array(&a.unwrap(), func());
        Ok(())
    }

    fn write_nullable_map<F, K, V, W>(
        &mut self,
        map: Option<HashMap<K, V>>,
        key_fn: F,
        value_fn: W,
    ) -> Result
    where
        F: FnMut(&K),
        W: FnMut(&V),
    {
        if map.is_none() {
            self.write_nil();
            return Ok(());
        }
        for (key, value) in map.unwrap() {
            self.write_map(map.unwrap(), key_fn(&key), value_fn(&value))
        }
        Ok(())
    }
}
