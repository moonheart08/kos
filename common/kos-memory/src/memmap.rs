use alloc::collections::LinkedList;

pub struct PhysicalMemoryMap {
    // I solemnly swear this really needs to be a linked list:
    // - Reallocations are unacceptable, especially early in bootstrapping when this is first constructed (as there's no room in memory.)
    // - Insertion needs to be relatively fast, as we modify this when "claiming" memory from the hardware to indicate we're now using that chunk.
    entries: LinkedList<MemoryMapEntry>
}

impl PhysicalMemoryMap {

}

pub struct MemoryMapEntry {
    pub base: u64,
    pub len: u64,

}

pub enum MemoryType {
    /// Free (i.e. kernel usable) memory.
    Free,
    /// Held by the kernel allocator.
    KernelAlloc,
    Reserved,
    AcpiReclaimable,
    AcpiNonvolatile,
    Bad,
    BootloaderReclaimable,
    Kernel,
    Framebuffer,
}