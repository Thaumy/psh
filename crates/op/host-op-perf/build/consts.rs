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

// [(major, patch_level, is_selected)]
#[rustfmt::skip]
pub const LINUX_FEATURE_VERSIONS: [(usize, usize, bool); 27] = [
    (6, 3, cfg!(feature = "linux-6.3" )),
    (6, 0, cfg!(feature = "linux-6.0" )),
    (5,16, cfg!(feature = "linux-5.16")),
    (5,13, cfg!(feature = "linux-5.13")),
    (5,12, cfg!(feature = "linux-5.12")),
    (5,11, cfg!(feature = "linux-5.11")),
    (5, 9, cfg!(feature = "linux-5.9" )),
    (5, 8, cfg!(feature = "linux-5.8" )),
    (5, 7, cfg!(feature = "linux-5.7" )),
    (5, 5, cfg!(feature = "linux-5.5" )),
    (5, 4, cfg!(feature = "linux-5.4" )),
    (5, 1, cfg!(feature = "linux-5.1" )),
    (4,17, cfg!(feature = "linux-4.17")),
    (4,16, cfg!(feature = "linux-4.16")),
    (4,14, cfg!(feature = "linux-4.14")),
    (4,12, cfg!(feature = "linux-4.12")),
    (4,10, cfg!(feature = "linux-4.10")),
    (4, 8, cfg!(feature = "linux-4.8" )),
    (4, 7, cfg!(feature = "linux-4.7" )),
    (4, 4, cfg!(feature = "linux-4.4" )),
    (4, 3, cfg!(feature = "linux-4.3" )),
    (4, 2, cfg!(feature = "linux-4.2" )),
    (4, 1, cfg!(feature = "linux-4.1" )),
    (3,19, cfg!(feature = "linux-3.19")),
    (3,16, cfg!(feature = "linux-3.16")),
    (3,13, cfg!(feature = "linux-3.13")),
    (3,12, cfg!(feature = "linux-3.12")),
];
