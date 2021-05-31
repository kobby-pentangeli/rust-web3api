pub struct Nullable<T> {
    value: T,
    nil: bool,
}

impl<T: std::cmp::PartialEq> Nullable<T> {
    pub fn constructor(&mut self) {
        self.set_to_null();
    }

    pub fn from_value(value: T) {}

    pub fn from_null() {}

    fn eq(&self, rhs: Nullable<T>) -> bool {
        if self.nil {
            self.nil == rhs.nil
        } else {
            !(rhs.nil || self.value != rhs.value)
        }
    }

    fn neq(&self, rhs: Nullable<T>) -> bool {
        !self.eq(rhs)
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, arg: T) {
        self.value = arg;
        self.nil = false;
    }

    pub fn is_null(&self) -> bool {
        self.nil
    }

    fn set_to_null(&mut self) {
        self.nil = true;
    }
}
