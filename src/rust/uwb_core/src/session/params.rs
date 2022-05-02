// Copyright 2022, The Android Open Source Project
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

pub mod ccc_app_config_params;
pub mod fira_app_config_params;
mod utils;

use std::collections::HashMap;

use crate::uci::params::{AppConfigTlv, AppConfigTlvType, SessionType};

/// The parameters of the UWB session.
#[derive(Debug, Clone)]
pub enum AppConfigParams {
    Fira(fira_app_config_params::FiraAppConfigParams),
    Ccc(ccc_app_config_params::CccAppConfigParams),
}

impl AppConfigParams {
    /// Generate the TLV list from the params.
    pub fn generate_tlvs(&self) -> Vec<AppConfigTlv> {
        Self::config_map_to_tlvs(self.generate_config_map())
    }

    /// Generate the updated TLV list from the difference between this and the previous params.
    pub fn generate_updated_tlvs(&self, prev_params: &Self) -> Vec<AppConfigTlv> {
        Self::config_map_to_tlvs(self.generate_updated_config_map(prev_params))
    }

    fn config_map_to_tlvs(config_map: HashMap<AppConfigTlvType, Vec<u8>>) -> Vec<AppConfigTlv> {
        config_map.into_iter().map(|(cfg_id, v)| AppConfigTlv { cfg_id, v }).collect()
    }

    fn generate_config_map(&self) -> HashMap<AppConfigTlvType, Vec<u8>> {
        match self {
            Self::Fira(params) => params.generate_config_map(),
            Self::Ccc(params) => params.generate_config_map(),
        }
    }

    fn generate_updated_config_map(
        &self,
        prev_params: &Self,
    ) -> HashMap<AppConfigTlvType, Vec<u8>> {
        match (self, prev_params) {
            (Self::Fira(params), Self::Fira(prev_params)) => {
                params.generate_updated_config_map(prev_params)
            }
            _ => HashMap::new(),
        }
    }

    pub fn is_type_matched(&self, session_type: SessionType) -> bool {
        match self {
            Self::Fira(_) => {
                session_type == SessionType::FiraDataTransfer
                    || session_type == SessionType::FiraRangingSession
            }
            Self::Ccc(_) => session_type == SessionType::Ccc,
        }
    }
}
