// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::VerifierConfig;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::vm_status::StatusCode;

/// Trait for a metering verification.
pub trait Meter {
    /// Add the number of units to the meter, returns an error if a limit is hit.
    fn add(&mut self, units: u128) -> PartialVMResult<()>;

    fn add_items(&mut self, units_per_item: u128, items: usize) -> PartialVMResult<()> {
        self.add(units_per_item.saturating_mul(items as u128))
    }
}

pub struct BoundingMeter {
    units: u128,
    max: Option<u128>,
}

pub struct DummyMeter;

impl Meter for BoundingMeter {
    fn add(&mut self, units: u128) -> PartialVMResult<()> {
        if let Some(max) = self.max {
            if self.units >= max - units {
                return Err(PartialVMError::new(StatusCode::CONSTRAINT_NOT_SATISFIED)
                    .with_message(format!(
                        "program too complex (`{} + {} > {}`)",
                        self.units, units, max
                    )));
            }
            self.units += units;
        }
        Ok(())
    }
}

impl BoundingMeter {
    pub fn new(config: &VerifierConfig) -> Self {
        Self {
            units: 0,
            max: config.max_meter_units,
        }
    }
}

impl Meter for DummyMeter {
    fn add(&mut self, _units: u128) -> PartialVMResult<()> {
        Ok(())
    }
}
