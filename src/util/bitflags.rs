pub trait Bitflags<T: Into<u8>> {
    fn register(&self) -> u8;
    fn set_register(&mut self, value: u8);

    fn flag(&self, flag: T) -> bool {
        return self.register() & flag.into() != 0;
    }

    fn set_flag(&mut self, flag: T, value: bool) {
        let value = if value {
            self.register() | flag.into()
        } else {
            self.register() & !flag.into()
        };
        self.set_register(value);
    }
}