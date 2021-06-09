use num_bigint::BigInt;
use std::collections::HashMap;

pub type Result = std::result::Result<(), failure::Error>;

pub trait Write {
    fn write_nil(&mut self);
    fn write_bool(&mut self, value: bool);
    fn write_i8(&mut self, value: i8);
    fn write_i16(&mut self, value: i16);
    fn write_i32(&mut self, value: i32);
    fn write_i64(&mut self, value: i64);
    fn write_u8(&mut self, value: u8);
    fn write_u16(&mut self, value: u16);
    fn write_u32(&mut self, value: u32);
    fn write_u64(&mut self, value: u64);
    fn write_f32(&mut self, value: f32);
    fn write_f64(&mut self, value: f64);
    fn write_string_length(&mut self, length: u32);
    fn write_string(&mut self, value: String);
    fn write_bytes_length(&mut self, length: u32);
    fn write_bytes(&mut self, buf: &[u8]) -> Result;
    fn write_bigint(&mut self, value: BigInt);
    fn write_array_length(&mut self, length: u32);
    fn write_array<T, F>(&mut self, a: &[T], func: F)
    where
        F: FnMut(u32);
    fn write_map_length(&mut self, length: u32);
    fn write_map<F, K, V, W>(&mut self, map: HashMap<K, V>, key_fn: F, value_fn: W)
    where
        F: FnMut(&K),
        W: FnMut(&V);
    fn write_nullable_bool(&mut self, value: Option<bool>) -> Result;
    fn write_nullable_i8(&mut self, value: Option<i8>) -> Result;
    fn write_nullable_i16(&mut self, value: Option<i16>) -> Result;
    fn write_nullable_i32(&mut self, value: Option<i32>) -> Result;
    fn write_nullable_i64(&mut self, value: Option<i64>) -> Result;
    fn write_nullable_u8(&mut self, value: Option<u8>) -> Result;
    fn write_nullable_u16(&mut self, value: Option<u16>) -> Result;
    fn write_nullable_u32(&mut self, value: Option<u32>) -> Result;
    fn write_nullable_u64(&mut self, value: Option<u64>) -> Result;
    fn write_nullable_f32(&mut self, value: Option<f32>) -> Result;
    fn write_nullable_f64(&mut self, value: Option<f64>) -> Result;
    fn write_nullable_string(&mut self, value: Option<String>) -> Result;
    fn write_nullable_bytes(&mut self, buf: Option<Vec<u8>>) -> Result;
    fn write_nullable_bigint(&mut self, value: Option<BigInt>) -> Result;
    fn write_nullable_array<T, F>(&mut self, a: Option<Vec<T>>, func: F) -> Result
    where
        F: FnMut(T);
    fn write_nullable_map<F, K, V, W>(
        &mut self,
        map: Option<HashMap<K, V>>,
        key_fn: F,
        value_fn: W,
    ) -> Result
    where
        F: FnMut(&K),
        W: FnMut(&V);
}
