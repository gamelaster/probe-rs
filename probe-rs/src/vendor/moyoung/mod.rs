//! Moyoung vendor support.

use probe_rs_target::Chip;

use crate::{config::DebugSequence, vendor::Vendor};

mod sequences;

/// Moyoung
#[derive(docsplay::Display)]
pub struct Moyoung;

impl Vendor for Moyoung {
    fn try_create_debug_sequence(&self, chip: &Chip) -> Option<DebugSequence> {
        if is_moy10x7(&chip.name) {
            Some(DebugSequence::Arm(sequences::Moy10x7::create()))
        } else {
            None
        }
    }
}

fn is_moy10x7(chip_name: &str) -> bool {
    chip_name.len() == "MOY10X7".len() && chip_name.starts_with("MOY10") && chip_name.ends_with('7')
}
