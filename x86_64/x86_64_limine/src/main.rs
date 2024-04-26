#![no_std]
#![no_main]

use limine::request::{BootloaderInfoRequest, FramebufferRequest, HhdmRequest, MemoryMapRequest, ModuleRequest, PagingModeRequest, StackSizeRequest};
use limine::BaseRevision;
mod memmap;

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
static BOOTLOADER_INFO_REQUEST: BootloaderInfoRequest = BootloaderInfoRequest::new();

#[used]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
static PAGING_MODE_REQUEST: PagingModeRequest = PagingModeRequest::new();

#[used]
static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[used]
pub static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(kos_memory::STACK_SIZE as u64);

#[no_mangle]
unsafe extern "C" fn _limine_entry() -> ! {
    assert!(BASE_REVISION.is_supported());
    assert!(MEMORY_MAP_REQUEST.get_response().is_some());
    assert!(HHDM_REQUEST.get_response().is_some());

    memmap::build_kernel_memory_map(MEMORY_MAP_REQUEST.get_response().unwrap());

    kos_kernel::halt();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    // If we panicked the system is probably cooked.
    kos_kernel::halt();
}

