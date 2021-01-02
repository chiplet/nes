mod test;
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
