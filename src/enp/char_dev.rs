use crate::bridge::dbridge::{DReader, DWriter};

pub struct CharDevRW {
    dev_path: String,
}

impl CharDevRW {
    pub fn new(dev_path: &str) -> Self {
        Self { dev_path: dev_path.to_owned() }
    }

    pub fn open(&self) -> bool {
        // simulate open
        println!("opening {}", self.dev_path);

        true
    }
}

impl DWriter for CharDevRW {}
impl DReader for CharDevRW {}

#[cfg(test)]
mod tests {
    use super::CharDevRW;

    #[test]
    fn bridge_construction() {
        let enp = CharDevRW::new("/dev/testdev");
        assert!(enp.open());
    }
}