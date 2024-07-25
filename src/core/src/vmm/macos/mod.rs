use self::cpu::HfCpu;
use self::vm::Vm;
use super::{MemoryAddr, Platform, Ram, VmmError};
use hv_sys::hv_vcpu_create;
use std::ffi::c_int;
use std::num::NonZero;
use std::sync::Arc;
use thiserror::Error;

mod cpu;
mod vm;

/// Implementation of [`Platform`] using Hypervisor Framework.
///
/// Fields in this struct need to drop in a correct order.
pub struct Hf {
    vm: Vm,
    ram: Arc<Ram>,
}

impl Hf {
    pub fn new(_: usize, ram: Arc<Ram>) -> Result<Self, VmmError> {
        // Create a VM.
        let vm = Vm::new().map_err(VmmError::CreateVmFailed)?;

        // Map memory.
        vm.vm_map(
            ram.host_addr().cast(),
            ram.vm_addr().try_into().unwrap(),
            ram.len(),
        )
        .map_err(VmmError::MapRamFailed)?;

        Ok(Self { vm, ram })
    }
}

impl Platform for Hf {
    type Cpu<'a> = HfCpu<'a>;
    type CpuErr = HfCpuError;

    fn create_cpu(&self, id: usize) -> Result<Self::Cpu<'_>, Self::CpuErr> {
        let mut instance = 0;
        #[cfg(target_arch = "aarch64")]
        let ret =
            unsafe { hv_vcpu_create(&mut instance, std::ptr::null_mut(), std::ptr::null_mut()) };
        #[cfg(target_arch = "x86_64")]
        let ret = unsafe { hv_vcpu_create(&mut instance, 0) };

        if let Some(e) = NonZero::new(ret) {
            return Err(HfCpuError::CreateVcpuFailed(e));
        }

        Ok(HfCpu::new(id, instance))
    }
}

/// Implementation of [`Platform::CpuErr`].
#[derive(Debug, Error)]
pub enum HfCpuError {
    #[error("couldn't create a vCPU ({0:#x})")]
    CreateVcpuFailed(NonZero<c_int>),
}