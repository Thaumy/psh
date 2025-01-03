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
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use super::MemInfo;

fn parse_meminfo_line(reader: BufReader<File>) -> io::Result<MemInfo> {
    let mut current_mem_info = MemInfo::new();
    for line in reader.lines() {
        let line = line?;
        let Some((key, value)) = line.trim().split_once(':') else {
            return Err(io::Error::other("No key/value pair in memory info."));
        };
        let value = value
            .trim()
            .split_once(' ')
            .and_then(|(num, _)| num.trim().parse().ok())
            .unwrap_or(0);
        match key {
            "MemTotal" => current_mem_info.mem_total = value,
            "MemFree" => current_mem_info.mem_free = value,
            "MemAvailable" => current_mem_info.mem_available = value,
            "Buffers" => current_mem_info.buffers = value,
            "Cached" => current_mem_info.cached = value,
            "SwapCached" => current_mem_info.swap_cached = value,
            "Active" => current_mem_info.active = value,
            "Inactive" => current_mem_info.inactive = value,
            "Active(anon)" => current_mem_info.active_anon = value,
            "Inactive(anon)" => current_mem_info.inactive_anon = value,
            "Active(file)" => current_mem_info.active_file = value,
            "Inactive(file)" => current_mem_info.inactive_file = value,
            "Unevictable" => current_mem_info.unevictable = value,
            "Mlocked" => current_mem_info.mlocked = value,
            "SwapTotal" => current_mem_info.swap_total = value,
            "SwapFree" => current_mem_info.swap_free = value,
            "Dirty" => current_mem_info.dirty = value,
            "Writeback" => current_mem_info.writeback = value,
            "AnonPages" => current_mem_info.anon_pages = value,
            "Mapped" => current_mem_info.mapped = value,
            "Shmem" => current_mem_info.shmem = value,
            "KReclaimable" => current_mem_info.kreclaimable = value,
            "Slab" => current_mem_info.slab = value,
            "SReclaimable" => current_mem_info.sreclaimable = value,
            "SUnreclaim" => current_mem_info.sunreclaim = value,
            "KernelStack" => current_mem_info.kernel_stack = value,
            "PageTables" => current_mem_info.page_tables = value,
            "NFS_Unstable" => current_mem_info.nfs_unstable = value,
            "Bounce" => current_mem_info.bounce = value,
            "WritebackTmp" => current_mem_info.writeback_tmp = value,
            "CommitLimit" => current_mem_info.commit_limit = value,
            "Committed_AS" => current_mem_info.committed_as = value,
            "VmallocTotal" => current_mem_info.vmalloc_total = value,
            "VmallocUsed" => current_mem_info.vmalloc_used = value,
            "VmallocChunk" => current_mem_info.vmalloc_chunk = value,
            "Percpu" => current_mem_info.percpu = value,
            "CmaTotal" => current_mem_info.cma_total = Some(value),
            "CmaFree" => current_mem_info.cma_free = Some(value),
            "HardwareCorrupted" => current_mem_info.hardware_corrupted = Some(value),
            "AnonHugePages" => current_mem_info.anon_huge_pages = Some(value),
            "ShmemHugePages" => current_mem_info.shmem_huge_pages = Some(value),
            "ShmemPmdMapped" => current_mem_info.shmem_pmd_mapped = Some(value),
            "FileHugePages" => current_mem_info.file_huge_pages = Some(value),
            "FilePmdMapped" => current_mem_info.file_pmd_mapped = Some(value),
            "HugePages_Total" => current_mem_info.huge_pages_total = value,
            "HugePages_Free" => current_mem_info.huge_pages_free = value,
            "HugePages_Rsvd" => current_mem_info.huge_pages_rsvd = value,
            "HugePages_Surp" => current_mem_info.huge_pages_surp = value,
            "Hugepagesize" => current_mem_info.huge_page_size = value,
            "Hugetlb" => current_mem_info.huge_tlb = value,
            "DirectMap4k" => current_mem_info.direct_map4k = Some(value),
            "DirectMap2M" => current_mem_info.direct_map2_m = Some(value),
            "DirectMap1G" => current_mem_info.direct_map1_g = Some(value),
            _ => (),
        }
    }
    Ok(current_mem_info)
}

#[allow(dead_code)]
pub fn do_parse_meminfo(path: &str) -> io::Result<MemInfo> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    parse_meminfo_line(reader)
}

#[allow(unused_macros)]
macro_rules! parse_meminfo {
    ($path:expr) => {
        crate::memory::mem_info::do_parse_meminfo($path)
    };
    () => {
        crate::memory::mem_info::do_parse_meminfo("/proc/meminfo")
    };
}

pub(crate) use parse_meminfo;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    #[expect(clippy::cognitive_complexity, reason = "test fn")]
    fn test_parse_meminfo_hed_off_cma_on() {
        let mut meminfo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        meminfo_path.push("./test_resources/memory/meminfo-hed-off-cma-on");
        let binding = meminfo_path.into_os_string();
        let meminfo_path = binding.to_str().unwrap();
        let result = parse_meminfo!(meminfo_path);

        // Match the result to handle the Result type correctly
        match result {
            Ok(mem_info) => {
                assert_eq!(mem_info.mem_total, 16215456);
                assert_eq!(mem_info.mem_free, 7305240);
                assert_eq!(mem_info.mem_available, 15612312);
                assert_eq!(mem_info.buffers, 156428);
                assert_eq!(mem_info.cached, 7872156);
                assert_eq!(mem_info.swap_cached, 0);
                assert_eq!(mem_info.active, 3993668);
                assert_eq!(mem_info.inactive, 4257628);
                assert_eq!(mem_info.active_anon, 1820);
                assert_eq!(mem_info.inactive_anon, 224124);
                assert_eq!(mem_info.active_file, 3991848);
                assert_eq!(mem_info.inactive_file, 4033504);
                assert_eq!(mem_info.unevictable, 64);
                assert_eq!(mem_info.mlocked, 64);
                assert_eq!(mem_info.swap_total, 0);
                assert_eq!(mem_info.swap_free, 0);
                assert_eq!(mem_info.dirty, 560);
                assert_eq!(mem_info.writeback, 0);
                assert_eq!(mem_info.anon_pages, 222804);
                assert_eq!(mem_info.mapped, 130992);
                assert_eq!(mem_info.shmem, 3232);
                assert_eq!(mem_info.kreclaimable, 442892);
                assert_eq!(mem_info.slab, 516968);
                assert_eq!(mem_info.sreclaimable, 442892);
                assert_eq!(mem_info.sunreclaim, 74076);
                assert_eq!(mem_info.kernel_stack, 3696);
                assert_eq!(mem_info.page_tables, 5040);
                assert_eq!(mem_info.nfs_unstable, 0);
                assert_eq!(mem_info.bounce, 0);
                assert_eq!(mem_info.writeback_tmp, 0);
                assert_eq!(mem_info.commit_limit, 8107728);
                assert_eq!(mem_info.committed_as, 1335652);
                assert_eq!(mem_info.vmalloc_total, 67108863);
                assert_eq!(mem_info.vmalloc_used, 20400);
                assert_eq!(mem_info.vmalloc_chunk, 0);
                assert_eq!(mem_info.percpu, 1760);
                assert_eq!(mem_info.cma_total, Some(327680));
                assert_eq!(mem_info.cma_free, Some(162368));
                assert!(mem_info.hardware_corrupted.is_none());
                assert!(mem_info.anon_huge_pages.is_none());
                assert!(mem_info.shmem_huge_pages.is_none());
                assert!(mem_info.shmem_pmd_mapped.is_none());
                assert!(mem_info.file_huge_pages.is_none());
                assert!(mem_info.file_pmd_mapped.is_none());
                assert_eq!(mem_info.huge_pages_total, 0);
                assert_eq!(mem_info.huge_pages_free, 0);
                assert_eq!(mem_info.huge_pages_rsvd, 0);
                assert_eq!(mem_info.huge_pages_surp, 0);
                assert_eq!(mem_info.huge_page_size, 2048);
                assert_eq!(mem_info.huge_tlb, 0);
                assert!(mem_info.direct_map4k.is_none());
                assert!(mem_info.direct_map2_m.is_none());
                assert!(mem_info.direct_map1_g.is_none());
            }
            Err(err) => {
                // Handle the error case if necessary
                panic!("Error parsing meminfo: {:?}", err);
            }
        }
    }

    #[test]
    #[expect(clippy::cognitive_complexity, reason = "test fn")]
    fn test_parse_meminfo_hed_on_cma_off() {
        let mut meminfo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        meminfo_path.push("./test_resources/memory/meminfo-hed-on-cma-off");
        let binding = meminfo_path.into_os_string();
        let meminfo_path = binding.to_str().unwrap();
        let result = parse_meminfo!(meminfo_path);

        // Match the result to handle the Result type correctly
        match result {
            Ok(mem_info) => {
                assert_eq!(mem_info.mem_total, 527952680);
                assert_eq!(mem_info.mem_free, 108170476);
                assert_eq!(mem_info.mem_available, 427200448);
                assert_eq!(mem_info.buffers, 2022300);
                assert_eq!(mem_info.cached, 306764708);
                assert_eq!(mem_info.swap_cached, 412432);
                assert_eq!(mem_info.active, 183149016);
                assert_eq!(mem_info.inactive, 215710720);
                assert_eq!(mem_info.active_anon, 1052840);
                assert_eq!(mem_info.inactive_anon, 90036200);
                assert_eq!(mem_info.active_file, 182096176);
                assert_eq!(mem_info.inactive_file, 125674520);
                assert_eq!(mem_info.unevictable, 27648);
                assert_eq!(mem_info.mlocked, 27648);
                assert_eq!(mem_info.swap_total, 33554424);
                assert_eq!(mem_info.swap_free, 22722580);
                assert_eq!(mem_info.dirty, 904);
                assert_eq!(mem_info.writeback, 0);
                assert_eq!(mem_info.anon_pages, 89683608);
                assert_eq!(mem_info.mapped, 23633960);
                assert_eq!(mem_info.shmem, 3042256);
                assert_eq!(mem_info.kreclaimable, 14685540);
                assert_eq!(mem_info.slab, 18107600);
                assert_eq!(mem_info.sreclaimable, 14685540);
                assert_eq!(mem_info.sunreclaim, 3422060);
                assert_eq!(mem_info.kernel_stack, 134288);
                assert_eq!(mem_info.page_tables, 1604488);
                assert_eq!(mem_info.nfs_unstable, 0);
                assert_eq!(mem_info.bounce, 0);
                assert_eq!(mem_info.writeback_tmp, 0);
                assert_eq!(mem_info.commit_limit, 297530764);
                assert_eq!(mem_info.committed_as, 737714324);
                assert_eq!(mem_info.vmalloc_total, 13743895347199);
                assert_eq!(mem_info.vmalloc_used, 430280);
                assert_eq!(mem_info.vmalloc_chunk, 0);
                assert_eq!(mem_info.percpu, 210560);
                assert!(mem_info.cma_total.is_none());
                assert!(mem_info.cma_free.is_none());
                assert_eq!(mem_info.hardware_corrupted, Some(0));
                assert_eq!(mem_info.anon_huge_pages, Some(569344));
                assert_eq!(mem_info.shmem_huge_pages, Some(0));
                assert_eq!(mem_info.shmem_pmd_mapped, Some(0));
                assert_eq!(mem_info.file_huge_pages, Some(0));
                assert_eq!(mem_info.file_pmd_mapped, Some(0));
                assert_eq!(mem_info.huge_pages_total, 0);
                assert_eq!(mem_info.huge_pages_free, 0);
                assert_eq!(mem_info.huge_pages_rsvd, 0);
                assert_eq!(mem_info.huge_pages_surp, 0);
                assert_eq!(mem_info.huge_page_size, 2048);
                assert_eq!(mem_info.huge_tlb, 0);
                assert_eq!(mem_info.direct_map4k, Some(7301120));
                assert_eq!(mem_info.direct_map2_m, Some(231403520));
                assert_eq!(mem_info.direct_map1_g, Some(299892736));
            }
            Err(err) => {
                // Handle the error case if necessary
                panic!("Error parsing meminfo: {:?}", err);
            }
        }
    }
}
