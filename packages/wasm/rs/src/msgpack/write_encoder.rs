use super::context::Context;
use super::data_view::DataView;
// use super::format::Format;
use super::write::{Result, Write};
use num_bigint::BigInt;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WriteEncoder {
    context: Context,
    view: DataView,
}

impl WriteEncoder {
    pub fn new(ua: &[u8], context: Context) -> Self {
        Self {
            context,
            view: DataView::new(ua).unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn get_context(&self) -> &Context {
        &self.context
    }
}

impl Write for WriteEncoder {
    fn write_nil(&mut self) {
        todo!()
    }

    fn write_bool(&mut self, _value: bool) {
        todo!()
    }

    fn write_i8(&mut self, _value: i8) {
        todo!()
    }

    fn write_i16(&mut self, _value: i16) {
        todo!()
    }

    fn write_i32(&mut self, _value: i32) {
        todo!()
    }

    fn write_i64(&mut self, _value: i64) {
        todo!()
    }

    fn write_u8(&mut self, _value: u8) {
        todo!()
    }

    fn write_u16(&mut self, _value: u16) {
        todo!()
    }

    fn write_u32(&mut self, _value: u32) {
        todo!()
    }

    fn write_u64(&mut self, _value: u64) {
        todo!()
    }

    fn write_f32(&mut self, _value: f32) {
        todo!()
    }

    fn write_f64(&mut self, _value: f64) {
        todo!()
    }

    fn write_string_length(&mut self, _length: u32) {
        todo!()
    }

    fn write_string(&mut self, _value: &str) {
        todo!()
    }

    fn write_bytes_length(&mut self, _length: u32) {
        todo!()
    }

    fn write_bytes(&mut self, _value: &[u8]) -> Result {
        todo!()
    }

    fn write_bigint(&mut self, _value: BigInt) {
        todo!()
    }

    fn write_array_length(&mut self, _length: u32) {
        todo!()
    }

    fn write_array<T, F>(&mut self, _a: &[T], _func: F)
    where
        F: FnMut(u32),
    {
        todo!()
    }

    fn write_map_length(&mut self, _length: u32) {
        todo!()
    }

    fn write_map<F, K, V, W>(&mut self, _map: HashMap<K, V>, _key_fnn: F, _value_fn: W)
    where
        F: FnMut(&K),
        W: FnMut(&V),
    {
        todo!()
    }

    fn write_nullable_bool(&mut self, _value: Option<bool>) -> Result {
        todo!()
    }

    fn write_nullable_i8(&mut self, _value: Option<i8>) -> Result {
        todo!()
    }

    fn write_nullable_i16(&mut self, _value: Option<i16>) -> Result {
        todo!()
    }

    fn write_nullable_i32(&mut self, _value: Option<i32>) -> Result {
        todo!()
    }

    fn write_nullable_i64(&mut self, _value: Option<i64>) -> Result {
        todo!()
    }

    fn write_nullable_u8(&mut self, _value: Option<u8>) -> Result {
        todo!()
    }

    fn write_nullable_u16(&mut self, _value: Option<u16>) -> Result {
        todo!()
    }

    fn write_nullable_u32(&mut self, _value: Option<u32>) -> Result {
        todo!()
    }

    fn write_nullable_u64(&mut self, _value: Option<u64>) -> Result {
        todo!()
    }

    fn write_nullable_f32(&mut self, _value: Option<f32>) -> Result {
        todo!()
    }

    fn write_nullable_f64(&mut self, _value: Option<f64>) -> Result {
        todo!()
    }

    fn write_nullable_string(&mut self, _value: Option<&str>) -> Result {
        todo!()
    }

    fn write_nullable_bytes(&mut self, _value: Option<Vec<u8>>) -> Result {
        todo!()
    }

    fn write_nullable_bigint(&mut self, _value: Option<BigInt>) -> Result {
        todo!()
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
