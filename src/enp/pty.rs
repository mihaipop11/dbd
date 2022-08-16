use std::io::{Write, Read};

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

impl Write for Pty {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for Pty {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Pty;

    #[test]
    fn bridge_construction() {
        let enp = Pty::new("/dev/testdev");
        assert!(enp.open());
    }
}