// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use risc0_zkvm::guest::env;
use zeth_lib::{block_builder::EthereumBlockBuilder, consts::ChainSpec};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Read the test block's chain specification
    let chain_spec: ChainSpec = env::read();
    // Read the input previous block and transaction data
    let input = env::read();
    // Build the resulting block
    let output = EthereumBlockBuilder::build_from(&chain_spec, input)
        .expect("Failed to build the resulting block");
    // Output the resulting block's hash to the journal
    env::commit(&output.hash());
    // Leak memory, save cycles
    core::mem::forget(output);
}
