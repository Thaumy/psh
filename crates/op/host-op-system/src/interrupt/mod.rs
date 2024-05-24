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

use std::time::Duration;

use crate::profiling::system::interrupt;
use crate::SysCtx;

use psh_system::interrupt::{InterruptDetails, InterruptType, IrqDetails};

impl From<&InterruptType> for interrupt::InterruptType {
    fn from(value: &InterruptType) -> Self {
        match value {
            InterruptType::Common(irq) => interrupt::InterruptType::Common(*irq),
            InterruptType::ArchSpecific(irq) => interrupt::InterruptType::ArchSpecific(irq.clone()),
        }
    }
}

impl From<InterruptType> for interrupt::InterruptType {
    fn from(value: InterruptType) -> Self {
        match value {
            InterruptType::Common(irq) => interrupt::InterruptType::Common(irq),
            InterruptType::ArchSpecific(irq) => interrupt::InterruptType::ArchSpecific(irq),
        }
    }
}

impl From<&IrqDetails> for interrupt::InterruptInfo {
    fn from(value: &IrqDetails) -> Self {
        Self {
            number: value.irq_number,
            smp_affinity: value.smp_affinity.clone(),
            smp_affinity_list: value.smp_affinity_list.clone(),
            node: value.node.clone(),
        }
    }
}

impl From<IrqDetails> for interrupt::InterruptInfo {
    fn from(value: IrqDetails) -> Self {
        Self {
            number: value.irq_number,
            smp_affinity: value.smp_affinity,
            smp_affinity_list: value.smp_affinity_list,
            node: value.node,
        }
    }
}

impl From<&InterruptDetails> for interrupt::InterruptStat {
    fn from(value: &InterruptDetails) -> Self {
        Self {
            interrupt_type: (&value.interrupt_type).into(),
            description: value.description.clone(),
            per_cpu_counts: value.cpu_counts.clone(),
        }
    }
}

impl From<InterruptDetails> for interrupt::InterruptStat {
    fn from(value: InterruptDetails) -> Self {
        Self {
            interrupt_type: value.interrupt_type.into(),
            description: value.description,
            per_cpu_counts: value.cpu_counts,
        }
    }
}

impl interrupt::Host for SysCtx {
    fn info(&mut self) -> wasmtime::Result<Result<Vec<interrupt::InterruptInfo>, String>> {
        let info = self
            .interrupt
            .info()
            .map(|ints| ints.into_iter().map(Into::into).collect())
            .map_err(|err| err.to_string());
        Ok(info)
    }

    fn stat(
        &mut self,
        interval_ms: u64,
    ) -> wasmtime::Result<Result<Vec<interrupt::InterruptStat>, String>> {
        let stat = self
            .interrupt
            .stat(Some(Duration::from_millis(interval_ms)))
            .map(|stats| stats.into_iter().map(Into::into).collect())
            .map_err(|err| err.to_string());
        Ok(stat)
    }
}
