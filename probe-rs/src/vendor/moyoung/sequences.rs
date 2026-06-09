//! Debug sequences for Moyoung targets.

use std::{sync::Arc, thread, time::Duration};

use probe_rs_target::CoreType;

use crate::{
    CoreStatus,
    architecture::arm::{ArmError, memory::ArmMemoryInterface, sequences::ArmDebugSequence},
};

const RESET_CONTROL: u64 = 0x4008_0004;
const SYSTEM_RESET: u64 = 0x4008_020c;
const SYSTEM_RESET_KEY: u32 = 0x55aa_0000;

/// Debug sequence for the Moyoung MOY10X7 family.
#[derive(Debug)]
pub struct Moy10x7;

impl Moy10x7 {
    /// Create the debug sequence.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl ArmDebugSequence for Moy10x7 {
    fn reset_system(
        &self,
        interface: &mut dyn ArmMemoryInterface,
        _core_type: CoreType,
        _debug_base: Option<u64>,
    ) -> Result<(), ArmError> {
        tracing::trace!("Running MOY10X7 system reset sequence");

        interface.write_word_32(RESET_CONTROL, 0x0)?;
        interface.flush()?;

        // The reset write can invalidate outstanding debug transactions on some probes.
        if let Err(error) = interface.write_word_32(SYSTEM_RESET, SYSTEM_RESET_KEY) {
            tracing::debug!("MOY10X7 reset write returned an error: {error}");
        }
        if let Err(error) = interface.flush() {
            tracing::debug!("MOY10X7 reset flush returned an error: {error}");
        }
        thread::sleep(Duration::from_millis(100));
        interface.update_core_status(CoreStatus::Unknown);

        Ok(())
    }
}
