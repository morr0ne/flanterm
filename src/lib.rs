#![no_std]

use core::ptr::{null_mut, NonNull};

use limone::requests::framebuffer::FrameBuffer;

#[allow(nonstandard_style)]
pub mod sys;

#[repr(transparent)]
pub struct Context {
    raw: NonNull<sys::flanterm_context>,
}

impl Context {
    pub fn from_framebuffer(framebuffer: &FrameBuffer) -> Option<Self> {
        unsafe {
            let raw = sys::flanterm_fb_init(
                None,
                None,
                framebuffer.address().cast(),
                framebuffer.width() as _,
                framebuffer.height() as _,
                framebuffer.pitch() as _,
                framebuffer.red_mask_size(),
                framebuffer.red_mask_shift(),
                framebuffer.green_mask_size(),
                framebuffer.green_mask_shift(),
                framebuffer.blue_mask_size(),
                framebuffer.blue_mask_shift(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                0,
                0,
                1,
                0,
                0,
                10,
            );

            if raw.is_null() {
                return None;
            }

            Some(Self {
                raw: NonNull::new_unchecked(raw),
            })
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        unsafe { sys::flanterm_write(self.raw.as_ptr(), buf.as_ptr().cast(), buf.len()) }
    }
}
