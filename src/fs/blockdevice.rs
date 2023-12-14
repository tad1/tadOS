//! adapted from embedded-sdmmc-rs - Block Device support
//!
//! Generic code for handling block devices.

/// Represents a standard 512 byte block (also known as a sector). IBM PC
/// formatted 5.25" and 3.5" floppy disks, SD/MMC cards up to 1 GiB in size
/// and IDE/SATA Hard Drives up to about 2 TiB all have 512 byte blocks.
///
/// This library does not support devices with a block size other than 512
/// bytes.
#[derive(Clone, Copy)]
pub struct Block {
    /// The 512 bytes in this block (or sector).
    pub contents: [u8; Block::LEN],
}

/// Represents the linear numeric address of a block (or sector). The first
/// block on a disk gets `BlockIdx(0)` (which usually contains the Master Boot
/// Record).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockIdx(pub u32);

/// Represents the a number of blocks (or sectors). Add this to a `BlockIdx`
/// to get an actual address on disk.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockCount(pub u32);

/// An iterator returned from `Block::range`.
pub struct BlockIter {
    inclusive_end: BlockIdx,
    current: BlockIdx,
}

/// Represents a block device - a device which can read and write blocks (or
/// sectors). Only supports devices which are <= 2 TiB in size.
pub trait BlockDevice {
    /// The errors that the `BlockDevice` can return. Must be debug formattable.
    type Error: core::fmt::Debug;
    /// Read one or more blocks, starting at the given block index.
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        reason: &str,
    ) -> Result<(), Self::Error>;
    /// Write one or more blocks, starting at the given block index.
    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error>;
    /// Determine how many blocks this device can hold.
    fn num_blocks(&self) -> Result<BlockCount, Self::Error>;
}

use super::fat::MAX_FAT_ENTRIES;

impl Block {
    /// All our blocks are a fixed length of 512 bytes. We do not support
    /// 'Advanced Format' Hard Drives with 4 KiB blocks, nor weird old
    /// pre-3.5-inch floppy disk formats.
    pub const LEN: usize = 512;

    /// Sometimes we want `LEN` as a `u32` and the casts don't look nice.
    pub const LEN_U32: u32 = 512;

    /// Create a new block full of zeros.
    pub fn new() -> Block {
        Block {
            contents: [0u8; Self::LEN],
        }
    }

    /// Converts a `&[[u8; 512]]` slice into a slice of `Block`s,
    /// assuming that there's no remainder.
    pub fn from_array_slice(blocks: &mut [[u8; Self::LEN]]) -> &mut [Block] {
        let len = blocks.len();
        // # Safety
        //
        // - The slice splits exactly into `N`-element chunks (aka `self.len() % N == 0`).
        // - `N != 0`
        unsafe { core::slice::from_raw_parts_mut(blocks.as_mut_ptr().cast::<Block>(), len) }
    }

    /// Converts a `&[[u8; 4]]` slice into a slice of `Block`s,
    /// assuming that there's no remainder.
    pub fn from_fat_entries(blocks: &mut [[u8; 4]]) -> &mut [Block] {
        let len = (blocks.len() * 4) / Block::LEN;
        // # Safety
        //
        // - The slice splits exactly into `N`-element chunks (aka `self.len() % N == 0`).
        // - `N != 0`
        unsafe { core::slice::from_raw_parts_mut(blocks.as_mut_ptr().cast::<Block>(), len) }
    }

    pub fn to_fat_entries(blocks: &mut [Block]) -> &mut [[u8; 4]] {
        let len = MAX_FAT_ENTRIES as usize;
        let ptr = (&mut blocks[0].contents).as_mut_ptr() as *mut [u8; 4];
        let buff;
        unsafe {
            // there is no way to turn a slice of arrays into a slice of bytes i.e.
            // turn a &mut [[u8; 512]] to a &mut [[u8; 4]] safely.
            // at least, we cannot do this without prior allocation. In a no_std environment, where
            // a heap doesnt exist, this becomes a pain. We'd either have to use something like `heapless`
            // (and predict the amount of space we'll use for allocation) or take the last resort.
            //
            // use `from_raw_parts_mut` to construct a mutable slice of bytes from a slice of arrays.
            //
            // # Safety
            // - This is still safe as it satifies `of from_raw_parts_mut` usage conditions.
            // - `Block's` are always multiples of 4.
            buff = core::slice::from_raw_parts_mut(ptr, len);
        }
        assert_eq!(blocks.len() * Block::LEN, buff.len() * 4);
        buff
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl core::ops::Add<BlockCount> for BlockIdx {
    type Output = BlockIdx;
    fn add(self, rhs: BlockCount) -> BlockIdx {
        BlockIdx(self.0 + rhs.0)
    }
}

impl core::ops::AddAssign<BlockCount> for BlockIdx {
    fn add_assign(&mut self, rhs: BlockCount) {
        self.0 += rhs.0
    }
}

impl core::ops::Add<BlockCount> for BlockCount {
    type Output = BlockCount;
    fn add(self, rhs: BlockCount) -> BlockCount {
        BlockCount(self.0 + rhs.0)
    }
}

impl core::ops::AddAssign<BlockCount> for BlockCount {
    fn add_assign(&mut self, rhs: BlockCount) {
        self.0 += rhs.0
    }
}

impl core::ops::Sub<BlockCount> for BlockIdx {
    type Output = BlockIdx;
    fn sub(self, rhs: BlockCount) -> BlockIdx {
        BlockIdx(self.0 - rhs.0)
    }
}

impl core::ops::SubAssign<BlockCount> for BlockIdx {
    fn sub_assign(&mut self, rhs: BlockCount) {
        self.0 -= rhs.0
    }
}

impl core::ops::Sub<BlockCount> for BlockCount {
    type Output = BlockCount;
    fn sub(self, rhs: BlockCount) -> BlockCount {
        BlockCount(self.0 - rhs.0)
    }
}

impl core::ops::SubAssign<BlockCount> for BlockCount {
    fn sub_assign(&mut self, rhs: BlockCount) {
        self.0 -= rhs.0
    }
}

impl core::ops::Deref for Block {
    type Target = [u8; 512];
    fn deref(&self) -> &[u8; 512] {
        &self.contents
    }
}

impl core::ops::DerefMut for Block {
    fn deref_mut(&mut self) -> &mut [u8; 512] {
        &mut self.contents
    }
}

impl core::fmt::Debug for Block {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeln!(fmt, "Block:")?;
        for line in self.contents.chunks(32) {
            for b in line {
                write!(fmt, "{:02x}", b)?;
            }
            write!(fmt, " ")?;
            for &b in line {
                if b >= 0x20 && b <= 0x7F {
                    write!(fmt, "{}", b as char)?;
                } else {
                    write!(fmt, ".")?;
                }
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

impl BlockIdx {
    /// Convert a block index into a 64-bit byte offset from the start of the
    /// volume. Useful if your underlying block device actually works in
    /// bytes, like `open("/dev/mmcblk0")` does on Linux.
    pub fn into_bytes(self) -> u64 {
        (u64::from(self.0)) * (Block::LEN as u64)
    }

    /// Create an iterator from the current `BlockIdx` through the given
    /// number of blocks.
    pub fn range(self, num: BlockCount) -> BlockIter {
        BlockIter::new(self, self + BlockCount(num.0))
    }
}

impl BlockCount {
    /// Take a number of blocks and increment by the integer number of blocks
    /// required to get to the block that holds the byte at the given offset.
    pub fn offset_bytes(self, offset: u32) -> Self {
        BlockCount(self.0 + (offset / Block::LEN_U32))
    }
}

impl BlockIter {
    /// Create a new `BlockIter`, from the given start block, through (and
    /// including) the given end block.
    pub fn new(start: BlockIdx, inclusive_end: BlockIdx) -> BlockIter {
        BlockIter {
            inclusive_end,
            current: start,
        }
    }
}

impl core::iter::Iterator for BlockIter {
    type Item = BlockIdx;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.0 >= self.inclusive_end.0 {
            None
        } else {
            let this = self.current;
            self.current += BlockCount(1);
            Some(this)
        }
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
