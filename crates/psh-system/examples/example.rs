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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network = psh_system::network::NetworkHandle::new();
    println!("{:#?}", network);
    let copy = network.clone();
    let duration = std::time::Duration::from_secs(1);
    let th = std::thread::spawn(move || {
        for _ in 0..6 {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let networks = copy.stat(Some(duration));
            println!("{:#?}", networks);
        }
    });
    for _ in 0..4 {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let networks = network.stat(Some(duration));
        println!("{:#?}", networks);
    }
    let result = th.join();
    println!("{:#?}", result);
    Ok(())
}
