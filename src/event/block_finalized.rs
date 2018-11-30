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

///! Event handling for "block reported" events.
use super::{Event, EventFactory};
use error::{Error, ErrorKind};
use ethabi::ParamType;
use web3::types::{Log, H256};

/// A factory that produces events of type "block finalized".
pub struct BlockFinalizedFactory {
    /// The log topic that matches the event that this factory produces.
    topic: H256,
}

impl Default for BlockFinalizedFactory {
    /// Instantiates a default topic.
    fn default() -> Self {
        BlockFinalizedFactory {
            // `sha3("BlockFinalised(bytes32)")`
            topic: "2b6cea6adc0c092ab654c32a0ee19b8ccddafbbc780bce0a5dd193bc30aa186e"
                .parse::<H256>()
                .unwrap(),
        }
    }
}

impl EventFactory for BlockFinalizedFactory {
    /// Returns the matching topic for "block finalized" events.
    fn topic(&self) -> H256 {
        self.topic
    }

    /// Tries to build a "Block Finalized" event from a log entry.
    ///
    /// # Arguments
    ///
    /// * `log` - The log that shall be converted into an event.
    fn from_log(&self, log: &Log) -> Result<Event, Error> {
        let log_data = ethabi::decode(&[ParamType::FixedBytes(32)], &log.data.0[..]);
        let block_hash: H256 = match log_data {
            // There should only be a single bytes32 in the vector of decoded elements.
            Ok(decoded_elements) => match super::block_hash_from_abi(&decoded_elements[0]) {
                Ok(block_hash) => block_hash,
                Err(error) => return Err(error),
            },
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::AbiError,
                    format!(
                        "Error when doing ABI decoding of 'block finalized' event: {}",
                        error
                    ),
                ))
            }
        };

        Ok(Event::BlockFinalized { block_hash })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ethabi;
    use ethabi::token::{StrictTokenizer, Tokenizer};
    use web3::types::Address;

    #[test]
    fn it_converts_logs_for_block_finalized() {
        let address = "1234567890123456789012345678901234567890"
            .parse::<Address>()
            .unwrap();

        let expected_block_hash =
            "a234567890123456789012345678901234567890123456789012345678901234";
        let tokens =
            StrictTokenizer::tokenize(&ethabi::ParamType::FixedBytes(32), expected_block_hash)
                .unwrap();

        let factory: BlockFinalizedFactory = Default::default();
        let log = super::super::test::build_log(address, vec![factory.topic()], &[tokens]);

        let event = factory.from_log(&log);
        match event {
            Ok(event) => match event {
                Event::BlockFinalized { block_hash } => {
                    assert_eq!(block_hash, expected_block_hash.parse::<H256>().unwrap())
                }
                _ => panic!("Extracted wrong type of event."),
            },
            Err(error) => panic!(
                "Unexpected error when building block finalized event from log: {}",
                error
            ),
        }
    }
}