use std::io::{Write, Read};

#[derive(Debug)]
pub struct DBridge<E1, E2>
where
    E1: Write + Read,
    E2: Read + Write,
{
    #[allow(dead_code)]
    enp1: E1,

    #[allow(dead_code)]
    enp2: E2,
}

impl<E1, E2> DBridge<E1, E2>
where
    E1: Write + Read,
    E2: Read + Write,
{
    pub fn new(enp1: E1, enp2: E2) -> Result<Self, &'static str> {
        Ok(DBridge {
            enp1,
            enp2,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::{DBridge};

    struct TestEndpointRO {}
    impl Read for TestEndpointRO {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            Ok(0)
        }
    }

    struct TestEndpointWO {}
    impl Write for TestEndpointWO {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct TestEndpointRW {}
    impl Write for TestEndpointRW {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    impl Read for TestEndpointRW {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            Ok(0)
        }
    }

    #[test]
    fn bridge_construction() {
        let endpoint1 = TestEndpointRW {};
        let endpoint2 = TestEndpointRW {};

        let bridge = DBridge::new(endpoint1, endpoint2);
        assert!(bridge.is_ok());
    }
}
