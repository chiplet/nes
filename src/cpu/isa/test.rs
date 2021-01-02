#[cfg(test)]
mod test {
    use crate::cpu::isa::{get_u8, get_u8_at, get_u16};

    #[test]
    fn get_u8_valid() {
        let bytes: [u8; 2] = [0x00, 0x23];
        let value = get_u8(&bytes).unwrap();
        assert_eq!(0x23, value);
    }

    #[test]
    #[should_panic]
    fn get_u8_invalid() {
        let bytes: [u8; 1] = [0x00];
        get_u8(&bytes).unwrap();
    }

    #[test]
    fn get_u8_at_valid() {
        let bytes: [u8; 3] = [0x00, 0xcd, 0xab];
        assert_eq!(0x00, get_u8_at(&bytes, 0).unwrap());
        assert_eq!(0xcd, get_u8_at(&bytes, 1).unwrap());
        assert_eq!(0xab, get_u8_at(&bytes, 2).unwrap());
    }

    #[test]
    fn get_u16_valid() {
        let bytes: [u8; 3] = [0x00, 0xcd, 0xab];
        let value = get_u16(&bytes).unwrap();
        assert_eq!(0xabcd, value);
    }
}