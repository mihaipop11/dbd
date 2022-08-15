use crate::bridge::dbridge::{DReader, DWriter};

#[derive(Debug)]
pub struct Pty {
    dev_path: String,
}

impl Pty {
    pub fn new(dev_path: &str) -> Self {
        Self { dev_path: dev_path.to_owned() }
    }

    pub fn open(&self) -> bool {
        // simulate open
        println!("opening {}", self.dev_path);

        true
    }
}

impl DWriter for Pty {}
impl DReader for Pty {}

#[cfg(test)]
mod tests {
    use super::Pty;

    #[test]
    fn bridge_construction() {
        let enp = Pty::new("/dev/testdev");
        assert!(enp.open());
    }
}