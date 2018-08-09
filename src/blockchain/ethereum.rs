// Copyright 2018 OpenST Ltd.
// Copyright 2018 OpenST Ltd.
// Copyright 2018 OpenST Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module implements the Blockchain trait for Ethereum.

extern crate web3;

use web3::futures::Future;

use super::Blockchain;

/// This struct implements the Blockchain trait.
pub struct Ethereum {
    address: String,
}

impl Ethereum {
    /// Creates a new instance of Ethereum pointing to the given address.
    pub fn new(address: String) -> Self {
        Ethereum { address }
    }
}

impl Blockchain for Ethereum {
    /// Uses web3 to retrieve the accounts.
    /// Converts them to hex notation and returns all accounts in a vector.
    fn get_accounts(&self) -> Vec<String> {
        let (_eloop, http) = web3::transports::Http::new(self.address.as_str()).unwrap();
        let web3 = web3::Web3::new(http);
        let accounts = web3.eth().accounts().wait().unwrap();

        let mut v = Vec::new();
        for account in accounts {
            v.push(format!("{:#x}", account));
        }

        v
    }
}

#[cfg(test)]
mod test {}
