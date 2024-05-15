// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

mod cpu;
mod disk;
pub mod error;
mod interrupt;
mod memory;
mod network;
mod os;
// mod process;
mod rps;
mod utils;

use std::collections::HashMap;

use cpu::CPUInfo;
use interrupt::{InterruptDetails, IrqDetails};
use memory::{MemInfo, MemoryModule};
use os::OsInfo;
pub use procfs::process::Process;
use procfs::{net::DeviceStatus, DiskStat};
use rps::RpsDetails;
use utils::Handle;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct System {
    pub page_size: u64,
    pub boot_time_sec: u64,
    pub tick_per_sec: u64,
    pub cpu_info_handle: Handle<CPUInfo>,
    pub disk_stat_handle: Handle<Vec<DiskStat>>,
    pub interrupt_info_handle: Handle<Vec<IrqDetails>>,
    pub interrupt_stat_handle: Handle<Vec<InterruptDetails>>,
    pub memory_info_handle: Handle<Vec<MemoryModule>>,
    pub memory_stat_handle: Handle<MemInfo>,
    pub network_stat_handle: Handle<HashMap<String, DeviceStatus>>,
    pub os_info_handle: Handle<OsInfo>,
    pub rps_info_handle: Handle<Vec<RpsDetails>>,
}

impl Default for System {
    fn default() -> Self {
        Self {
            page_size: procfs::page_size(),
            boot_time_sec: procfs::boot_time_secs().unwrap_or(0),
            tick_per_sec: procfs::ticks_per_second(),
            cpu_info_handle: cpu::global::info_handle(),
            disk_stat_handle: disk::global::stat_handle(),
            interrupt_info_handle: interrupt::global::info_handle(),
            interrupt_stat_handle: interrupt::global::stat_handle(),
            memory_info_handle: memory::global::info_handle(),
            memory_stat_handle: memory::global::stat_handle(),
            network_stat_handle: network::global::stat_handle(),
            os_info_handle: os::global::info_handle(),
            rps_info_handle: rps::global::info_handle(),
        }
    }
}
