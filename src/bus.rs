use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

/// Abstraction of the NES system bus
pub struct Bus {
    devices: Vec<Box<dyn BusDevice>>
}
impl Bus {
    /// Initialize an empty `Bus`
    fn new() -> Self {
        Bus { devices: vec![] }
    }

    /// Add a `BusDevice` to this `Bus`
    fn add(&mut self, device: Box<dyn BusDevice>) -> Result<(), String> {
        // make sure that no address ranges overlap
        for d in self.devices.iter() {
            if (**d).get_addr_range().start <= (*device).get_addr_range().end
                && (*device).get_addr_range().start <= (**d).get_addr_range().end {
                return Err(
                    format!(
                        "Address range {} of new device overlaps with existing range {}",
                        (*device).get_addr_range(), (**d).get_addr_range()
                    )
                )
            }
        }
        self.devices.push(device);
        Ok(())
    }

    /// Get a reference to `BusDevice` mapped to given address
    fn get_mapped_device(&self, addr: u16) -> Result<&Box<dyn BusDevice>, String> {
        self.devices
            .iter()
            .find(|x| x.get_addr_range().start <= addr && addr <= x.get_addr_range().end)
            .ok_or(format!("No mapped address range covers address: {}", addr))
    }

    /// Get a mutable reference to `BusDevice` mapped to given address
    fn get_mut_mapped_device(&mut self, addr: u16) -> Result<&mut Box<dyn BusDevice>, String> {
        self.devices
            .iter_mut()
            .find(|x| x.get_addr_range().start <= addr && addr <= x.get_addr_range().end)
            .ok_or(format!("No mapped address range covers address: {}", addr))
    }

    /// Read a single byte from bus address `addr`
    fn read(&self, addr: u16) -> Result<u8, String> {
        let device = self.get_mapped_device(addr)?;
        Ok((*device).read_from_bus(addr))
    }

    /// Write a single byte to bus address `addr`
    fn write(&mut self, addr: u16, data: u8) -> Result<(), String> {
        let mut device = self.get_mut_mapped_device(addr)?;
        device.write_from_bus(addr, data);
        Ok(())
    }
}

/// A device connected to the system bus `Bus`
pub trait BusDevice {
    /// Read a single byte from bus address `addr`
    /// `Bus` calls this function when it wants to read from a particular device
    fn read_from_bus(&self, addr: u16) -> u8;

    /// Write a single byte `data` to bus address `addr`
    /// `Bus` calls this function when it wants to write to a particular device
    fn write_from_bus(&mut self, addr: u16, data: u8);

    /// Get address range associated with the device
    fn get_addr_range(&self) -> &AddrRange;
}

/// Bus address range (inclusive) assigned to a device.
pub struct AddrRange {
    pub start: u16,
    pub end: u16,
}
impl AddrRange {
    fn new(start: u16, end: u16) -> Self {
        AddrRange { start, end }
    }

    /// Map given address to collection index starting from 0
    fn address_to_index(&self, addr: u16) -> usize {
        (addr - self.start) as usize
    }
}
impl fmt::Display for AddrRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[${:04X}, ${:04X}]", self.start, self.end)
    }
}

/// `BusDevice` representing a contiguous block of random access memory
struct RamDevice {
    bus: Rc<RefCell<Bus>>,      // Bus this device is connected to
    addr_range: AddrRange,      // Bus address range mapped to this device
    memory: Vec<u8>,            // Bytes stored in the device
}
impl RamDevice {
    fn new(bus: &Rc<RefCell<Bus>>, start: usize, size: usize) -> Box<RamDevice> {
        Box::new(
            RamDevice {
                bus: Rc::clone(&bus),
                addr_range: AddrRange {
                    start: start as u16,
                    end: (start + size - 1) as u16,
                },
                memory: vec![0; size],
            }
        )
    }

    fn fill_with_test_data(&mut self) {
        for (i, num) in self.memory.iter_mut().enumerate() {
            *num = (i % 256) as u8;
        }
    }
}
impl BusDevice for RamDevice {
    fn read_from_bus(&self, addr: u16) -> u8 {
        self.memory[self.addr_range.address_to_index(addr)]
    }

    fn write_from_bus(&mut self, addr: u16, data: u8) {
        self.memory[self.addr_range.address_to_index(addr)] = data;
    }

    fn get_addr_range(&self) -> &AddrRange {
        &self.addr_range
    }
}

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