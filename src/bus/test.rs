#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::bus::{Bus, BusDevice, RamDevice, AddrRange};

    #[test]
    // RamDevice filling up the entire memory map
    fn read_from_ram_device() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));

        // init devices
        let mut ram = RamDevice::new(&bus, 0, 2_usize.pow(16));
        ram.fill_with_test_data();

        // add devices to bus
        bus.borrow_mut().add(ram);

        for i in 0..2_usize.pow(16) {
            assert_eq!(bus.borrow().read(i as u16).unwrap(), (i % 256) as u8);
        }
    }

    #[test]
    // 1KB RamDevice starting at address 0x0800
    fn read_from_ram_device_with_offset() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));

        let start: usize = 0x0800;
        let size: usize = 2_usize.pow(10);

        // init devices
        let mut ram = RamDevice::new(&bus, start, size);
        ram.fill_with_test_data();

        // add devices to bus
        bus.borrow_mut().add(ram);

        for i in start..start+size {
            assert_eq!(bus.borrow().read(i as u16).unwrap(), (i % 256) as u8);
        }
    }

    #[test]
    fn write_to_ram_device() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));

        let start: usize = 0x0800;
        let size: usize = 2_usize.pow(10);

        // init devices
        let mut ram = RamDevice::new(&bus, start, size);

        // add devices to bus
        bus.borrow_mut().add(ram);

        // write bytes to mapped memory
        for addr in start..start+size {
            bus.borrow_mut().write(addr as u16, (addr % 256) as u8).unwrap();
        }

        // read bytes from mapped memory
        for addr in start..start+size {
            assert_eq!(bus.borrow().read(addr as u16).unwrap(), (addr % 256) as u8);
        }
    }

    #[test]
    // legal memory mapping where ram1 < ram2
    fn disallow_overlapping_memory_regions_1() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        let mut ram1 = RamDevice::new(&bus, 0x0000, 0x0200);
        let mut ram2 = RamDevice::new(&bus, 0x0200, 0x0200);
        bus.borrow_mut().add(ram1).unwrap();
        bus.borrow_mut().add(ram2).unwrap();
    }

    #[test]
    #[should_panic]
    // illegal memory mapping where ram1 <= ram2
    fn disallow_overlapping_memory_regions_2() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        let mut ram1 = RamDevice::new(&bus, 0x0000, 0x0201);
        let mut ram2 = RamDevice::new(&bus, 0x0200, 0x0200);
        bus.borrow_mut().add(ram1).unwrap();
        bus.borrow_mut().add(ram2).unwrap();    // should panic
    }

    #[test]
    #[should_panic]
    // illegal memory mapping where ram1 <= ram2
    fn disallow_overlapping_memory_regions_3() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        let mut ram1 = RamDevice::new(&bus, 0x0000, 0x0300);
        let mut ram2 = RamDevice::new(&bus, 0x0200, 0x0200);
        bus.borrow_mut().add(ram1).unwrap();
        bus.borrow_mut().add(ram2).unwrap();    // should panic
    }

    #[test]
    #[should_panic]
    // illegal memory mapping where ram2 is enclosed in ram1
    fn disallow_overlapping_memory_regions_4() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        let mut ram1 = RamDevice::new(&bus, 0x0200, 0x0200);
        let mut ram2 = RamDevice::new(&bus, 0x0300, 0x0080);
        bus.borrow_mut().add(ram1).unwrap();
        bus.borrow_mut().add(ram2).unwrap();    // should panic
    }

    #[test]
    #[should_panic]
    // illegal memory mapping where ram1 >= ram2
    fn disallow_overlapping_memory_regions_5() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        let mut ram1 = RamDevice::new(&bus, 0x0300, 0x0200);
        let mut ram2 = RamDevice::new(&bus, 0x0200, 0x0200);
        bus.borrow_mut().add(ram1).unwrap();
        bus.borrow_mut().add(ram2).unwrap();    // should panic
    }

    #[test]
    #[should_panic]
    // illegal memory mapping where ram1 >= ram2
    fn disallow_overlapping_memory_regions_6() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        let mut ram1 = RamDevice::new(&bus, 0x0400, 0x0200);
        let mut ram2 = RamDevice::new(&bus, 0x0200, 0x0201);
        bus.borrow_mut().add(ram1).unwrap();
        bus.borrow_mut().add(ram2).unwrap();    // should panic
    }

    #[test]
    // legal memory mapping where ram1 > ram2
    fn disallow_overlapping_memory_regions_7() {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        let mut ram1 = RamDevice::new(&bus, 0x0400, 0x0200);
        let mut ram2 = RamDevice::new(&bus, 0x0200, 0x0200);
        bus.borrow_mut().add(ram1).unwrap();
        bus.borrow_mut().add(ram2).unwrap();
    }
}