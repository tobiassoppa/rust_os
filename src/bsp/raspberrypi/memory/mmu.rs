// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

//! BSP Memory Management Unit.

use super::map as memory_map;
use crate::memory::mmu::*;
use core::ops::RangeInclusive;

//
// Public Definitions
//

const NUM_MEM_RANGES: usize = 2;

/// The virtual memory layout.
///
/// The layout must contain only special ranges, aka anything that is _not_
/// normal cacheable DRAM. It is agnostic of the paging granularity that the
/// architecture's MMU will use.
pub static LAYOUT: KernelVirtualLayout<{ NUM_MEM_RANGES }> = KernelVirtualLayout::new(
    memory_map::END_INCLUSIVE,
    [
        TranslationDescriptor {
            name: "Kernel code and RO data",
            virtual_range: ro_range_inclusive,
            physical_range_translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadOnly,
                execute_never: false,
            },
        },
        TranslationDescriptor {
            name: "Device MMIO",
            virtual_range: mmio_range_inclusive,
            physical_range_translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            },
        },
    ],
);

//
// Private Code
//

fn ro_range_inclusive() -> RangeInclusive<usize> {
    // Notice the subtraction to turn the exclusive end into an inclusive end.
    #[allow(clippy::range_minus_one)]
    RangeInclusive::new(super::ro_start(), super::ro_end() - 1)
}

fn mmio_range_inclusive() -> RangeInclusive<usize> {
    RangeInclusive::new(memory_map::mmio::START, memory_map::mmio::END_INCLUSIVE)
}

//
// Public Code
//

/// Return the address space size in bytes.
pub const fn addr_space_size() -> usize {
    memory_map::END_INCLUSIVE + 1
}

/// Return a reference to the virtual memory layout.
pub fn virt_mem_layout() -> &'static KernelVirtualLayout<{ NUM_MEM_RANGES }> {
    &LAYOUT
}
