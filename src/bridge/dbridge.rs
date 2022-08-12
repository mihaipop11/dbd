pub trait DReader {}

pub trait DWriter {}

pub struct DBridge<E1, E2>
where
    E1: DWriter + DReader,
    E2: DReader + DWriter,
{
    #[allow(dead_code)]
    enp1: E1,

    #[allow(dead_code)]
    enp2: E2,
}

impl<E1, E2> DBridge<E1, E2>
where
    E1: DWriter + DReader,
    E2: DReader + DWriter,
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
    use super::{DBridge, DReader, DWriter};

    struct TestEndpointRO {}
    impl DReader for TestEndpointRO {}

    struct TestEndpointWO {}
    impl DWriter for TestEndpointWO {}

    struct TestEndpointRW {}
    impl DWriter for TestEndpointRW {}
    impl DReader for TestEndpointRW {}

    #[test]
    fn bridge_construction() {
        let endpoint1 = TestEndpointRW {};
        let endpoint2 = TestEndpointRW {};

        let bridge = DBridge::new(endpoint1, endpoint2);
        assert!(bridge.is_ok());
    }
}
