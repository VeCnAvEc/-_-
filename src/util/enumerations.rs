pub mod enumerations {
    pub fn forVec(latter_bytes: Vec<u8>) -> Vec<u8> {
        let mut bytes = vec![];

        for el in latter_bytes {
            bytes.push(el);
        }

        bytes
    }
}