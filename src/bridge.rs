trait DReader {}

trait DWriter {}

#[derive(Debug)]
pub struct DBridge {
    dev1: String,
    dev2: String,
}

impl DBridge {
    pub fn new<'a>(dev1: &'a str, dev2: &'a str) -> Result<Self, &'a str> {
        if dev1 == dev2 {
            Err("Failed to create bridge, duplicated device name given.")
        } else {
            Ok(DBridge {
                dev1: dev1.to_string(),
                dev2: dev2.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DBridge;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn bridge_construction_unique_devices() {
        let d1 = "device1";
        let d2 = "device2";

        let bridge = DBridge::new(d1, d2);
        assert!(bridge.is_ok());
    }
}