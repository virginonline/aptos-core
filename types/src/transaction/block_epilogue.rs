// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::state_store::state_key::StateKey;
use aptos_crypto::HashValue;
use derive_more::Deref;
#[cfg(any(test, feature = "fuzzing"))]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Debug, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub enum BlockEpiloguePayload {
    V0 {
        block_id: HashValue,
        block_end_info: BlockEndInfo,
    },
    V1 {
        block_id: HashValue,
        block_end_info: BlockEndInfo,
        fee_distribution: FeeDistribution,
    },
}

impl BlockEpiloguePayload {
    pub fn try_as_block_end_info(&self) -> Option<&BlockEndInfo> {
        match self {
            BlockEpiloguePayload::V0 { block_end_info, .. } => Some(block_end_info),
            BlockEpiloguePayload::V1 { block_end_info, .. } => Some(block_end_info),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(Arbitrary))]
pub enum FeeDistribution {
    V0 {
        // Validator index -> Octa
        amount: BTreeMap<u64, u64>,
    },
}

impl FeeDistribution {
    pub fn new(amount: BTreeMap<u64, u64>) -> Self {
        Self::V0 { amount }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockEndInfo {
    V0 {
        /// Whether block gas limit was reached
        block_gas_limit_reached: bool,
        /// Whether block output limit was reached
        block_output_limit_reached: bool,
        /// Total gas_units block consumed
        block_effective_block_gas_units: u64,
        /// Total output size block produced
        block_approx_output_size: u64,
    },
}

impl BlockEndInfo {
    pub fn new_empty() -> Self {
        Self::V0 {
            block_gas_limit_reached: false,
            block_output_limit_reached: false,
            block_effective_block_gas_units: 0,
            block_approx_output_size: 0,
        }
    }

    pub fn limit_reached(&self) -> bool {
        match self {
            BlockEndInfo::V0 {
                block_gas_limit_reached,
                block_output_limit_reached,
                ..
            } => *block_gas_limit_reached || *block_output_limit_reached,
        }
    }

    pub fn block_effective_gas_units(&self) -> u64 {
        match self {
            BlockEndInfo::V0 {
                block_effective_block_gas_units,
                ..
            } => *block_effective_block_gas_units,
        }
    }
}

/// Wrapper type to temporarily host the hot_state_ops which will not serialize until
/// the hot state is made entirely deterministic
/// TODO(HotState): maybe get rid of this struct now that it doesn't have anything more than
/// `BlockEndInfo`?
#[derive(Debug, Deref)]
pub struct TBlockEndInfoExt<Key: Debug> {
    #[deref]
    inner: BlockEndInfo,
    _phantom: PhantomData<Key>,
}

pub type BlockEndInfoExt = TBlockEndInfoExt<StateKey>;

impl<Key: Debug> TBlockEndInfoExt<Key> {
    pub fn new_empty() -> Self {
        Self {
            inner: BlockEndInfo::new_empty(),
            _phantom: PhantomData,
        }
    }

    pub fn new(inner: BlockEndInfo) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub fn to_persistent(&self) -> BlockEndInfo {
        self.inner.clone()
    }
}
