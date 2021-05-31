use crate::Nullable;
use num_bigint::BigInt;
use std::collections::HashMap;
use std::io::Result;

pub trait Read {
    fn read_bool(&mut self) -> Result<bool>;
    fn read_i8(&mut self) -> Result<i8>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i32(&mut self) -> Result<i32>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u64(&mut self) -> Result<u64>;
    fn read_f32(&mut self) -> Result<f32>;
    fn read_f64(&mut self) -> Result<f64>;
    fn read_string_length(&mut self) -> Result<u32>;
    fn read_string(&mut self) -> Result<&str>;
    fn read_bytes_length(&mut self) -> Result<u32>;
    fn read_bytes(&mut self) -> Result<Vec<u8>>;
    fn read_bigint(&mut self) -> Result<BigInt>;
    fn read_array_length(&mut self) -> Result<u32>;
    fn read_array<T, F>(&mut self, func: F) -> Result<Vec<T>>
    where
        F: FnMut(u32);
    fn read_map_length(&mut self) -> Result<u32>;
    fn read_map<F, K, V, W>(&mut self, key_fn: F, value_fn: W) -> HashMap<K, V>
    where
        F: FnMut(&K),
        W: FnMut(&V);
    fn read_nullable_bool(&mut self) -> Result<Nullable<bool>>;
    fn read_nullable_i8(&mut self) -> Result<Nullable<i8>>;
    fn read_nullable_i16(&mut self) -> Result<Nullable<i16>>;
    fn read_nullable_i32(&mut self) -> Result<Nullable<i32>>;
    fn read_nullable_i64(&mut self) -> Result<Nullable<i64>>;
    fn read_nullable_u8(&mut self) -> Result<Nullable<u8>>;
    fn read_nullable_u16(&mut self) -> Result<Nullable<u16>>;
    fn read_nullable_u32(&mut self) -> Result<Nullable<u32>>;
    fn read_nullable_u64(&mut self) -> Result<Nullable<u64>>;
    fn read_nullable_f32(&mut self) -> Result<Nullable<f32>>;
    fn read_nullable_f64(&mut self) -> Result<Nullable<f64>>;
    fn read_nullable_string(&mut self) -> Option<&str>;
    fn read_nullable_bytes(&mut self) -> Option<Vec<u8>>;
    fn read_nullable_bigint(&mut self) -> Option<BigInt>;
    fn read_nullable_array<T, F>(&mut self, func: F) -> Option<Vec<T>>
    where
        F: FnMut(T);
    fn read_nullable_map<F, K, V, W>(&mut self, key_fn: F, value_fn: W) -> Option<HashMap<K, V>>
    where
        F: FnMut(&K),
        W: FnMut(&V);
}
