// based on https://doc.rust-lang.org/std/pin/index.html

use std::{
    marker::PhantomPinned,
    pin::{pin, Pin},
};

#[derive(Default)]
struct SafeAddrTracker {
    prev_addr: Option<usize>,
    // remove auto-implemented `Unpin` bound to mark this type as having some
    // address-sensitive state. This is essential for our expected pinning
    // guarantees to work, and is discussed more below.
    _pin: PhantomPinned,
}

impl SafeAddrTracker {
    fn check_for_move(self: Pin<&mut Self>) {
        let current_addr = &*self as *const Self as usize;
        match self.prev_addr {
            None => {
                // SAFETY: we do not move out of self
                let self_data_mut = unsafe { self.get_unchecked_mut() };
                self_data_mut.prev_addr = Some(current_addr);
            }
            Some(prev_addr) => assert_eq!(prev_addr, current_addr),
        }
    }
}

#[derive(Default)]
struct AddrTracker(Option<usize>);

impl AddrTracker {
    // If we haven't checked the addr of self yet, store the current
    // address. If we have, confirm that the current address is the same
    // as it was last time, or else panic.
    fn check_for_move(&mut self) {
        let current_addr = self as *mut Self as usize;
        match self.0 {
            None => self.0 = Some(current_addr),
            Some(prev_addr) => assert_eq!(prev_addr, current_addr),
        }
    }
}

fn main() {
    // 1. Create the value, not yet in an address-sensitive state
    let tracker = SafeAddrTracker::default();

    // 2. Pin the value by putting it behind a pinning pointer, thus putting
    // it into an address-sensitive state
    let mut ptr_to_pinned_tracker: Pin<&mut SafeAddrTracker> = pin!(tracker);
    ptr_to_pinned_tracker.as_mut().check_for_move();

    // Trying to access `tracker` or pass `ptr_to_pinned_tracker` to anything that requires
    // mutable access to a non-pinned version of it will no longer compile

    // 3. We can now assume that the tracker value will never be moved, thus
    // this will never panic!
    ptr_to_pinned_tracker.as_mut().check_for_move();

    // Create a tracker and store the initial address
    let mut tracker = AddrTracker::default();
    tracker.check_for_move();

    // Here we shadow the variable. This carries a semantic move, and may therefore also
    // come with a mechanical memory *move*
    let mut tracker = tracker;

    tracker.check_for_move();
}
