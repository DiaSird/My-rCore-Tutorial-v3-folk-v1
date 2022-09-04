//! Implementation of physical and virtual address and page number.

use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};
use core::fmt::{self, Debug, Formatter};

/// physical address
///
/// SV39 supports a physical address bit width of 56 bits,
/// so only the lower 56 bits of usize are used when generating PhysAddr.
const PA_WIDTH_SV39: usize = 56;
/// virtual address width
const VA_WIDTH_SV39: usize = 39;
/// physical address number width
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
/// virtual address number width
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

// Definitions

/// # physical address(SV39: 56bit)
///
/// | BitNum  |55----------------12|11---------0|
/// |---------|--------------------|------------|
/// | Meaning | PhysicalPageNumber | PageOffset |
/// |  Width  |         44         |     12     |
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

/// # Virtual address(SV39: 39bit)
///
/// | BitNum  |38----------------12|11---------0|
/// |---------|--------------------|------------|
/// | Meaning | VirtualPageNumber  | PageOffset |
/// |  Width  |         27         |     12     |
///
/// # Virtual page number
/// - SV39: 39(VirtAddr) - 12(offset) = 27bit
///
/// | Meaning | VPN2 | VPN1  | VPN0 |
/// |---------|------|-------|------|
/// |  Width  |   9  |   9   |   9  |
///
/// - VPN2: Index of the 3rd-level page table.
///   - To find the physical page number of the 2nd-level page table
///     in the physical page of the 3rd-level page table with VPN2 as the offset.
///
/// - VPN1: Index of the 2nd-level page table.
///   - To find the physical page number of the 1st-level page table
///     in the physical page of the 2nd-level page table with VPN1 as the offset.
///
/// - VPN0: Index of the 1st-level page table.
///   - To find the physical page number of the accessed location
///     in the physical page of the 1st-level page table, using VPN0 as the offset.
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

/// # Physical page number
/// - SV39: 56(PhisAddr) - 12(offset) = 44bit
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

/// # Virtual page number
/// - SV39: 39(VirtAddr) - 12(offset) = 27bit
///
/// | Meaning | VPN2 | VPN1  | VPN0 |
/// |---------|------|-------|------|
/// |  Width  |   9  |   9   |   9  |
///
/// - VPN2: Index of the 3rd-level page table.
///   - To find the physical page number of the 2nd-level page table
///     in the physical page of the 3rd-level page table with VPN2 as the offset.
///
/// - VPN1: Index of the 2nd-level page table.
///   - To find the physical page number of the 1st-level page table
///     in the physical page of the 2nd-level page table with VPN1 as the offset.
///
/// - VPN0: Index of the 1st-level page table.
///   - To find the physical page number of the accessed location
///     in the physical page of the 1st-level page table, using VPN0 as the offset.
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

/// Debugging

impl Debug for VirtAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}

impl Debug for VirtPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}

impl Debug for PhysAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl Debug for PhysPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
    }
}

/// T: {PhysAddr, VirtAddr, PhysPageNum, VirtPageNum}
/// T -> usize: T.0
/// usize -> T: usize.into()

impl From<usize> for PhysAddr {
    /// Create a PhysAddr structure storing only PA_WIDTH_SV39(56bit).
    fn from(v: usize) -> Self {
        // e.g. (1 << 3) - 1 = 0b111
        // e.g. (1 << 4) - 1 = 0b1111
        // e.g. (1 << 5) - 1 = 0b11111
        // This & (logical product) yields only the trailing digit of the shift.
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    /// Create a PhysPageNum structure storing only PPN_WIDTH_SV39(44bit).
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtAddr {
    /// Create a VirtAddr structure storing only VA_WIDTH_SV39(39bit).
    fn from(v: usize) -> Self {
        Self(v & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtPageNum {
    /// Create a VirtPageNum structure storing only VPN_WIDTH_SV39(27bit).
    fn from(v: usize) -> Self {
        Self(v & ((1 << VPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}

impl From<VirtAddr> for usize {
    /// If VirtAddr fits into 38 digits, return as is.
    ///
    ///  If not, set all bits above the 39th digit to 1 before returning.
    fn from(v: VirtAddr) -> Self {
        // Over (VA_WIDTH_SV39(39) - 1) = 38 digits?
        if v.0 >= (1 << (VA_WIDTH_SV39 - 1)) {
            // 39th digit ~ usize(RV64 is 64) all bits in digit 1.
            v.0 | (!((1 << VA_WIDTH_SV39) - 1))
        } else {
            v.0
        }
    }
}

impl From<VirtPageNum> for usize {
    fn from(v: VirtPageNum) -> Self {
        v.0
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(v: VirtAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<VirtPageNum> for VirtAddr {
    /// `VirtPageNum` by 2**`PAGE_SIZE_BITS(12)` to get `VirtAddr`
    fn from(v: VirtPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    /// Increments by 1 for every PAGE_SIZE(4096)
    /// # Examples
    ///
    /// - If `PAGE_SIZE` is 4096
    ///
    /// ```rust
    /// // PhisAddr(8192)
    /// let phis_address = PhisAddr::from(4096 * 2);
    /// // (4096 * 2) − 1 + 4096) / 4096
    /// let phis_page_num = phis_address.ceil();
    /// assert_eq!(phis_page_num.0, 2);
    ///
    /// // PhisAddr(8194)
    /// let phis_address = PhisAddr::from(4097 * 2);
    /// // (4097 * 2) − 1 + 4096) / 4096
    /// let phis_page_num = phis_address.ceil();
    /// assert_eq!(phis_page_num.0, 3);
    /// ```
    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    /// Is the Physical Address aligned to a multiple of PAGE_SIZE (default: 4096)?
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}
