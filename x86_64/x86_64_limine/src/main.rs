#![no_std]
#![no_main]

use limine::request::FramebufferRequest;
use limine::BaseRevision;

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
unsafe extern "C" fn _limine_entry() -> ! {
    assert!(BASE_REVISION.is_supported());

    kos_kernel::halt();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    kos_kernel::halt();
}

