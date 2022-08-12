use crate::bridge::dbridge::{DReader, DWriter};

#[derive(Debug)]
pub struct FileEndpointRW {
    dev_path: String,
}

impl FileEndpointRW {
    pub fn new(dev_path: &str) -> Self {
        Self { dev_path: dev_path.to_owned() }
    }

    pub fn open(&self) -> bool {
        // simulate open
        println!("opening {}", self.dev_path);

        true
    }
}

impl DWriter for FileEndpointRW {}
impl DReader for FileEndpointRW {}

#[cfg(test)]
mod tests {
    use super::FileEndpointRW;

    #[test]
    fn bridge_construction() {
        let enp = FileEndpointRW::new("/dev/testdev");
        assert!(enp.open());
    }
}