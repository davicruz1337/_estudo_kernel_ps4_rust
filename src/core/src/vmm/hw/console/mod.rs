use self::context::Context;
use super::{Device, DeviceContext, Ram, PAGE_SIZE};
use obvirt::console::MsgType;
use std::collections::VecDeque;
use std::num::NonZero;
use std::sync::Mutex;

mod context;

/// Virtual console for the VM.
pub struct Console {
    addr: usize,
    logs: Mutex<VecDeque<Log>>,
}

impl Console {
    pub(crate) const SIZE: NonZero<usize> = PAGE_SIZE;

    pub fn new(addr: usize) -> Self {
        Self {
            addr,
            logs: Mutex::default(),
        }
    }
}

impl Device for Console {
    fn addr(&self) -> usize {
        self.addr
    }

    fn len(&self) -> NonZero<usize> {
        Self::SIZE
    }

    fn create_context<'a>(&'a self, ram: &'a Ram) -> Box<dyn DeviceContext + 'a> {
        Box::new(Context::new(self, ram))
    }
}

/// Contains data for each logging entry.
struct Log {
    ty: MsgType,
    file: String,
    line: u32,
    msg: String,
}
