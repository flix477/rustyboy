pub trait Register {
    fn get(&self) -> u8;
    fn set(&mut self, value: u8);
}