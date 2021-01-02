#[cfg(test)]
mod test {
    use crate::cpu::{BitOps, CPU, CARRY_BIT, OVERFLOW_BIT};

    #[test]
    fn get_bit() {
        let r = 0x55;
        assert_eq!(1, r.get_bit(0));
        assert_eq!(0, r.get_bit(1));
        assert_eq!(1, r.get_bit(2));
        assert_eq!(0, r.get_bit(3));
        assert_eq!(1, r.get_bit(4));
        assert_eq!(0, r.get_bit(5));
        assert_eq!(1, r.get_bit(6));
        assert_eq!(0, r.get_bit(7));
    }

    #[test]
    fn set_bit() {
        let mut r = 0x00;
        r.set_bit(0);
        r.set_bit(2);
        r.set_bit(4);
        r.set_bit(6);

        assert_eq!(0x55, r);
    }

    #[test]
    fn clear_bit() {
        let mut r = 0xff;
        r.clear_bit(0);
        r.clear_bit(2);
        r.clear_bit(4);
        r.clear_bit(6);

        assert_eq!(0xaa, r);
    }

    #[test]
    fn assign_bit() {
        let mut r = 0;
        r.assign_bit(0, 1);
        assert_eq!(r, 1);
        r.assign_bit(0, 0);
        assert_eq!(r, 0);

        let mut r = 0xaa;               // 1010_1010
        r.assign_bit(7, 0);     // 0010_1010
        assert_eq!(r, 0x2a);
        r.assign_bit(6, 1);     // 0110_1010
        assert_eq!(r, 0x6a);
        r.assign_bit(4, 1);     // 0111_1010
        assert_eq!(r, 0x7a);
    }

    #[test]
    fn adc_carry_flag() {
        let mut cpu = CPU::init();

        cpu.load_hexdump("./hexdumps/tests/adc_carry_test.txt").unwrap();
        cpu.pc = 0x0600;

        // CLC, LDA #$FF, ADC #$01
        // sum: 1111_1111 + 0000_0001 (should carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1);
        assert_eq!(cpu.a, 0x00);

        // CLC, LDA #$80, ADC #$80
        // sum: 1000_000 + 1000_0000 (should carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1);
        assert_eq!(cpu.a, 0x00);

        // CLC, LDA #$C0, ADC #$40
        // sum: 1100_000 + 0100_0000 (should carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1);
        assert_eq!(cpu.a, 0x00);

        // SEC, LDA #$fe, ADC #$01
        // sum: 1111_1110 + 0000_0001 (should carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1);
        assert_eq!(cpu.a, 0x00);

        // CLC, LDA #$80, ADC #$40
        // sum: 1000_000 + 0100_0000 (should not carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 0);
        assert_eq!(cpu.a, 0xc0);
    }

    #[test]
    fn adc_overflow_flag() {
        let mut cpu = CPU::init();

        cpu.load_hexdump("./hexdumps/tests/adc_overflow_test.txt").unwrap();
        cpu.pc = 0x0600;

        // CLC, LDA #$50, ADC #$50
        // 80 + 80 = 160 > 127 (should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0xa0);

        // CLC, LDA #$7f, ADC #$01
        // 127 + 1 = 128 > 127 (should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0x80);

        // SEC, LDA #$7f, ADC #$00
        // 127 + 0 + 1 = 128 > 127 (should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0x80);

        // CLC, LDA #$7e, ADC #$00
        // 126 + 1 = 127 <= 127 (should not set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 0);
        assert_eq!(cpu.a, 0x7f);
    }

    #[test]
    fn sbc_carry_flag() {
        let mut cpu = CPU::init();

        cpu.load_hexdump("./hexdumps/tests/sbc_overflow_test.txt").unwrap();
        cpu.pc = 0x0600;

        // SEC, LDA #$50, SBC #$b0
        // 80 - -80 = -96 (should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0xa0);
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1u8 - 1);

        // SEC, LDA #$d0, SBC #$70
        // -48 - 112 = 96 >(should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0x60);
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1u8 - 0);

        // SEC, LDA #$50, SBC #$f0
        // 80 - -16 = 96 >(should not set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 0);
        assert_eq!(cpu.a, 0x60);
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1u8 - 1);
    }

    #[test]
    fn functional_test() {
        // TODO: Add asserts
        let mut cpu = CPU::init();

        cpu.load_ines("./hexdumps/tests/nestest.nes").unwrap();
        cpu.pc = 0xc000;
        loop {
            cpu.tick();
        }
    }
}