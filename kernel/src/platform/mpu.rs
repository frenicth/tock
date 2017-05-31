#[derive(Debug)]
pub enum AccessPermission {
    //                                 Privileged  Unprivileged
    //                                 Access      Access
    NoAccess = 0b000, //.............. --          --
    PrivilegedOnly = 0b001, //........ RW          --
    UnprivilegedReadOnly = 0b010, //.. RW          R-
    ReadWrite = 0b011, //............. RW          RW
    Reserved = 0b100, //.............. undef       undef
    PrivilegedOnlyReadOnly = 0b101, // R-          --
    ReadOnly = 0b110, //.............. R-          R-
    ReadOnlyAlais = 0b111, //......... R-          R-
}

#[derive(Debug)]
pub enum ExecutePermission {
    ExecutionPermitted = 0b0,
    ExecutionNotPermitted = 0b1,
}

pub struct Region {
    base_address: u32,
    attributes: u32,
}

impl Region {
    pub unsafe fn new(base_address: u32, attributes: u32) -> Region {
        Region {
            base_address: base_address,
            attributes: attributes,
        }
    }

    pub fn empty() -> Region {
        Region {
            base_address: 0,
            attributes: 0,
        }
    }

    pub fn base_address(&self) -> u32 {
        self.base_address
    }

    pub fn attributes(&self) -> u32 {
        self.attributes
    }
}

pub trait MPU {
    /// Enables MPU, allowing privileged software access to the default memory
    /// map.
    fn enable_mpu(&self);

    /// Creates a new MPU-specific memory protection region
    ///
    /// `region_num`: an MPU region number 0-7
    /// `start_addr`: the region base address. Lower bits will be masked
    ///               according to the region size.
    /// `len`       : region size as a PowerOfTwo (e.g. `16` for 64KB)
    /// `execute`   : whether to enable code execution from this region
    /// `ap`        : access permissions as defined in Table 4.47 of the user
    ///               guide.
    fn create_region(region_num: usize,
                     start: usize,
                     len: usize,
                     execute: ExecutePermission,
                     access: AccessPermission)
                     -> Option<Region>;

    /// Sets the base address, size and access attributes of the given MPU
    /// region number.
    fn set_mpu(&self, region: Region);
}

/// Noop implementation of MPU trait
impl MPU for () {
    fn enable_mpu(&self) {}

    fn create_region(_: usize,
                     _: usize,
                     _: usize,
                     _: ExecutePermission,
                     _: AccessPermission)
                     -> Option<Region> {
        Some(Region::empty())
    }

    fn set_mpu(&self, _: Region) {}
}
