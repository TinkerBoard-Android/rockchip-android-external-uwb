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

//! Provide the conversion between the uwb_core's elements and protobuf bindings.

use std::convert::{TryFrom, TryInto};

use protobuf::RepeatedField;
use zeroize::Zeroize;

use crate::error::{Error, Result};
use crate::params::fira_app_config_params::{
    AoaResultRequest, BprfPhrDataRate, DeviceRole, DeviceType, FiraAppConfigParams,
    FiraAppConfigParamsBuilder, HoppingMode, KeyRotation, MacAddressMode, MacFcsType,
    MultiNodeMode, PreambleDuration, PrfMode, PsduDataRate, RangeDataNtfConfig,
    RangingRoundControl, RangingRoundUsage, RangingTimeStruct, ResultReportConfig, RframeConfig,
    ScheduledMode, StsConfig, StsLength, TxAdaptivePayloadPower, UwbAddress, UwbChannel,
};
use crate::params::uci_packets::{
    Controlee, DeviceState, ExtendedAddressTwoWayRangingMeasurement, PowerStats,
    RangingMeasurementType, ReasonCode, SessionState, SessionType,
    ShortAddressTwoWayRangingMeasurement, StatusCode, UpdateMulticastListAction,
};
use crate::params::AppConfigParams;
use crate::proto::bindings::{
    AoaResultRequest as ProtoAoaResultRequest, BprfPhrDataRate as ProtoBprfPhrDataRate,
    Controlee as ProtoControlee, DeviceRole as ProtoDeviceRole, DeviceState as ProtoDeviceState,
    DeviceType as ProtoDeviceType, FiraAppConfigParams as ProtoFiraAppConfigParams,
    HoppingMode as ProtoHoppingMode, KeyRotation as ProtoKeyRotation,
    MacAddressMode as ProtoMacAddressMode, MacFcsType as ProtoMacFcsType,
    MultiNodeMode as ProtoMultiNodeMode, PowerStats as ProtoPowerStats,
    PreambleDuration as ProtoPreambleDuration, PrfMode as ProtoPrfMode,
    PsduDataRate as ProtoPsduDataRate, RangeDataNtfConfig as ProtoRangeDataNtfConfig,
    RangingMeasurement as ProtoRangingMeasurement,
    RangingMeasurementType as ProtoRangingMeasurementType,
    RangingRoundControl as ProtoRangingRoundControl, RangingRoundUsage as ProtoRangingRoundUsage,
    RangingTimeStruct as ProtoRangingTimeStruct, ReasonCode as ProtoReasonCode,
    ResultReportConfig as ProtoResultReportConfig, RframeConfig as ProtoRframeConfig,
    ScheduledMode as ProtoScheduledMode, SessionRangeData as ProtoSessionRangeData,
    SessionState as ProtoSessionState, SessionType as ProtoSessionType, Status as ProtoStatus,
    StatusCode as ProtoStatusCode, StsConfig as ProtoStsConfig, StsLength as ProtoStsLength,
    TxAdaptivePayloadPower as ProtoTxAdaptivePayloadPower, UciLoggerMode as ProtoUciLoggerMode,
    UpdateMulticastListAction as ProtoUpdateMulticastListAction, UwbChannel as ProtoUwbChannel,
};
use crate::uci::notification::{RangingMeasurements, SessionRangeData};
use crate::uci::uci_logger::UciLoggerMode;

/// Generate the conversion functions between 2 enum types, which field is 1-to-1 mapping.
///
/// Example:
/// ```
/// enum EnumA {
///     Value1,
///     Value2,
/// }
/// enum EnumB {
///     Foo,
///     Bar,
/// }
/// // This macro generates `From<EnumA> for EnumB` and `From<EnumB> for EnumA`.
/// uwb_core::enum_mapping! {
///     EnumA => EnumB,
///     Value1 => Foo,
///     Value2 => Bar,
/// }
/// ```
#[macro_export]
macro_rules! enum_mapping {
    ( $enum_a:ty => $enum_b:ty, $( $field_a:ident => $field_b:ident, )+ ) => {
        impl From<$enum_a> for $enum_b {
            fn from(item: $enum_a) -> $enum_b {
                match item {
                    $(
                        <$enum_a>::$field_a => <$enum_b>::$field_b,
                    )*
                }
            }
        }
        impl From<$enum_b> for $enum_a {
            fn from(item: $enum_b) -> $enum_a {
                match item {
                    $(
                        <$enum_b>::$field_b => <$enum_a>::$field_a,
                    )*
                }
            }
        }
    };
}

enum_mapping! {
    ProtoStatusCode => StatusCode,
    UCI_STATUS_OK => UciStatusOk,
    UCI_STATUS_REJECTED => UciStatusRejected,
    UCI_STATUS_FAILED => UciStatusFailed,
    UCI_STATUS_SYNTAX_ERROR => UciStatusSyntaxError,
    UCI_STATUS_INVALID_PARAM => UciStatusInvalidParam,
    UCI_STATUS_INVALID_RANGE => UciStatusInvalidRange,
    UCI_STATUS_INVALID_MSG_SIZE => UciStatusInvalidMsgSize,
    UCI_STATUS_UNKNOWN_GID => UciStatusUnknownGid,
    UCI_STATUS_UNKNOWN_OID => UciStatusUnknownOid,
    UCI_STATUS_READ_ONLY => UciStatusReadOnly,
    UCI_STATUS_COMMAND_RETRY => UciStatusCommandRetry,
    UCI_STATUS_SESSION_NOT_EXIST => UciStatusSessionNotExist,
    UCI_STATUS_SESSION_DUPLICATE => UciStatusSessionDuplicate,
    UCI_STATUS_SESSION_ACTIVE => UciStatusSessionActive,
    UCI_STATUS_MAX_SESSIONS_EXCEEDED => UciStatusMaxSessionsExceeded,
    UCI_STATUS_SESSION_NOT_CONFIGURED => UciStatusSessionNotConfigured,
    UCI_STATUS_ACTIVE_SESSIONS_ONGOING => UciStatusActiveSessionsOngoing,
    UCI_STATUS_MULTICAST_LIST_FULL => UciStatusMulticastListFull,
    UCI_STATUS_ADDRESS_NOT_FOUND => UciStatusAddressNotFound,
    UCI_STATUS_ADDRESS_ALREADY_PRESENT => UciStatusAddressAlreadyPresent,
    UCI_STATUS_RANGING_TX_FAILED => UciStatusRangingTxFailed,
    UCI_STATUS_RANGING_RX_TIMEOUT => UciStatusRangingRxTimeout,
    UCI_STATUS_RANGING_RX_PHY_DEC_FAILED => UciStatusRangingRxPhyDecFailed,
    UCI_STATUS_RANGING_RX_PHY_TOA_FAILED => UciStatusRangingRxPhyToaFailed,
    UCI_STATUS_RANGING_RX_PHY_STS_FAILED => UciStatusRangingRxPhyStsFailed,
    UCI_STATUS_RANGING_RX_MAC_DEC_FAILED => UciStatusRangingRxMacDecFailed,
    UCI_STATUS_RANGING_RX_MAC_IE_DEC_FAILED => UciStatusRangingRxMacIeDecFailed,
    UCI_STATUS_RANGING_RX_MAC_IE_MISSING => UciStatusRangingRxMacIeMissing,
    UCI_STATUS_ERROR_ROUND_INDEX_NOT_ACTIVATED => UciStatusErrorRoundIndexNotActivated,
    UCI_STATUS_ERROR_NUMBER_OF_ACTIVE_RANGING_ROUNDS_EXCEEDED =>
        UciStatusErrorNumberOfActiveRangingRoundsExceeded,
    UCI_STATUS_ERROR_ROUND_INDEX_NOT_SET_AS_INITIATOR => UciStatusErrorRoundIndexNotSetAsInitiator,
    UCI_STATUS_ERROR_DL_TDOA_DEVICE_ADDRESS_NOT_MATCHING_IN_REPLY_TIME_LIST =>
        UciStatusErrorDlTdoaDeviceAddressNotMatchingInReplyTimeList,
    UCI_STATUS_DATA_MAX_TX_PSDU_SIZE_EXCEEDED => UciStatusDataMaxTxPsduSizeExceeded,
    UCI_STATUS_DATA_RX_CRC_ERROR => UciStatusDataRxCrcError,
    UCI_STATUS_ERROR_CCC_SE_BUSY => UciStatusErrorCccSeBusy,
    UCI_STATUS_ERROR_CCC_LIFECYCLE => UciStatusErrorCccLifecycle,
}

enum_mapping! {
    ProtoDeviceState => DeviceState,
    DEVICE_STATE_READY => DeviceStateReady,
    DEVICE_STATE_ACTIVE => DeviceStateActive,
    DEVICE_STATE_ERROR => DeviceStateError,
}

enum_mapping! {
    ProtoSessionState => SessionState,
    INIT => SessionStateInit,
    DEINIT => SessionStateDeinit,
    ACTIVE => SessionStateActive,
    IDLE => SessionStateIdle,
}

enum_mapping! {
    ProtoReasonCode => ReasonCode,
    STATE_CHANGE_WITH_SESSION_MANAGEMENT_COMMANDS => StateChangeWithSessionManagementCommands,
    MAX_RANGING_ROUND_RETRY_COUNT_REACHED => MaxRangingRoundRetryCountReached,
    MAX_NUMBER_OF_MEASUREMENTS_REACHED => MaxNumberOfMeasurementsReached,
    ERROR_SLOT_LENGTH_NOT_SUPPORTED => ErrorSlotLengthNotSupported,
    ERROR_INSUFFICIENT_SLOTS_PER_RR => ErrorInsufficientSlotsPerRr,
    ERROR_MAC_ADDRESS_MODE_NOT_SUPPORTED => ErrorMacAddressModeNotSupported,
    ERROR_INVALID_RANGING_INTERVAL => ErrorInvalidRangingInterval,
    ERROR_INVALID_STS_CONFIG => ErrorInvalidStsConfig,
    ERROR_INVALID_RFRAME_CONFIG => ErrorInvalidRframeConfig,
}

enum_mapping! {
    ProtoUciLoggerMode => UciLoggerMode,
    UCI_LOGGER_MODE_DISABLED => Disabled,
    UCI_LOGGER_MODE_UNFILTERED => Unfiltered,
    UCI_LOGGER_MODE_FILTERED => Filtered,
}

enum_mapping! {
    ProtoRangingMeasurementType => RangingMeasurementType,
    ONE_WAY => OneWay,
    TWO_WAY => TwoWay,
    DL_TDOA => DlTdoa,
}

enum_mapping! {
    ProtoSessionType => SessionType,
    FIRA_RANGING_SESSION => FiraRangingSession,
    FIRA_DATA_TRANSFER => FiraDataTransfer,
    CCC => Ccc,
}

enum_mapping! {
    ProtoDeviceType => DeviceType,
    CONTROLEE => Controlee,
    CONTROLLER => Controller,
}

enum_mapping! {
    ProtoRangingRoundUsage => RangingRoundUsage,
    SS_TWR => SsTwr,
    DS_TWR => DsTwr,
    SS_TWR_NON => SsTwrNon,
    DS_TWR_NON => DsTwrNon,
}

enum_mapping! {
    ProtoStsConfig => StsConfig,
    STATIC => Static,
    DYNAMIC => Dynamic,
    DYNAMIC_FOR_CONTROLEE_INDIVIDUAL_KEY => DynamicForControleeIndividualKey,
}

enum_mapping! {
    ProtoMultiNodeMode => MultiNodeMode,
    UNICAST => Unicast,
    ONE_TO_MANY => OneToMany,
    MANY_TO_MANY => ManyToMany,
}

enum_mapping! {
    ProtoUwbChannel => UwbChannel,
    CHANNEL_5 => Channel5,
    CHANNEL_6 => Channel6,
    CHANNEL_8 => Channel8,
    CHANNEL_9 => Channel9,
    CHANNEL_10 => Channel10,
    CHANNEL_12 => Channel12,
    CHANNEL_13 => Channel13,
    CHANNEL_14 => Channel14,
}

enum_mapping! {
    ProtoMacFcsType => MacFcsType,
    CRC_16 => Crc16,
    CRC_32 => Crc32,
}

enum_mapping! {
    ProtoAoaResultRequest => AoaResultRequest,
    NO_AOA_REPORT => NoAoaReport,
    REQ_AOA_RESULTS => ReqAoaResults,
    REQ_AOA_RESULTS_AZIMUTH_ONLY => ReqAoaResultsAzimuthOnly,
    REQ_AOA_RESULTS_ELEVATION_ONLY => ReqAoaResultsElevationOnly,
    REQ_AOA_RESULTS_INTERLEAVED => ReqAoaResultsInterleaved,
}

enum_mapping! {
    ProtoRangeDataNtfConfig => RangeDataNtfConfig,
    RANGE_DATA_NTF_CONFIG_DISABLE => Disable,
    RANGE_DATA_NTF_CONFIG_ENABLE => Enable,
    RANGE_DATA_NTF_CONFIG_ENABLE_PROXIMITY => EnableProximity,
}

enum_mapping! {
    ProtoDeviceRole => DeviceRole,
    RESPONDER => Responder,
    INITIATOR => Initiator,
}

enum_mapping! {
    ProtoRframeConfig => RframeConfig,
    SP0 => SP0,
    SP1 => SP1,
    SP3 => SP3,
}

enum_mapping! {
    ProtoPsduDataRate => PsduDataRate,
    RATE_6M_81 => Rate6m81,
    RATE_7M_80 => Rate7m80,
    RATE_27M_2 => Rate27m2,
    RATE_31M_2 => Rate31m2,
    RATE_850K => Rate850k,
}

enum_mapping! {
    ProtoPreambleDuration => PreambleDuration,
    T32_SYMBOLS => T32Symbols,
    T64_SYMBOLS => T64Symbols,
}

enum_mapping! {
    ProtoRangingTimeStruct => RangingTimeStruct,
    INTERVAL_BASED_SCHEDULING => IntervalBasedScheduling,
    BLOCK_BASED_SCHEDULING => BlockBasedScheduling,
}

enum_mapping! {
    ProtoTxAdaptivePayloadPower => TxAdaptivePayloadPower,
    TX_ADAPTIVE_PAYLOAD_POWER_DISABLE => Disable,
    TX_ADAPTIVE_PAYLOAD_POWER_ENABLE => Enable,
}

enum_mapping! {
    ProtoPrfMode => PrfMode,
    BPRF => Bprf,
    HPRF_WITH_124_8_MHZ => HprfWith124_8MHz,
    HPRF_WITH_249_6_MHZ => HprfWith249_6MHz,
}

enum_mapping! {
    ProtoScheduledMode => ScheduledMode,
    TIME_SCHEDULED_RANGING => TimeScheduledRanging,
}

enum_mapping! {
    ProtoKeyRotation => KeyRotation,
    KEY_ROTATION_DISABLE => Disable,
    KEY_ROTATION_ENABLE => Enable,
}

enum_mapping! {
    ProtoMacAddressMode => MacAddressMode,
    MAC_ADDRESS_2_BYTES => MacAddress2Bytes,
    MAC_ADDRESS_8_BYTES_2_BYTES_HEADER => MacAddress8Bytes2BytesHeader,
    MAC_ADDRESS_8_BYTES => MacAddress8Bytes,
}

enum_mapping! {
    ProtoHoppingMode => HoppingMode,
    HOPPING_MODE_DISABLE => Disable,
    FIRA_HOPPING_ENABLE => FiraHoppingEnable,
}

enum_mapping! {
    ProtoBprfPhrDataRate => BprfPhrDataRate,
    BPRF_PHR_DATA_RATE_850K => Rate850k,
    BPRF_PHR_DATA_RATE_6M_81 => Rate6m81,
}

enum_mapping! {
    ProtoStsLength => StsLength,
    LENGTH_32 => Length32,
    LENGTH_64 => Length64,
    LENGTH_128 => Length128,
}

enum_mapping! {
    ProtoUpdateMulticastListAction => UpdateMulticastListAction,
    ADD_CONTROLEE => AddControlee,
    REMOVE_CONTROLEE => RemoveControlee,
}

impl<T> From<Result<T>> for ProtoStatus {
    fn from(item: Result<T>) -> Self {
        match item {
            Ok(_) => Self::OK,
            Err(Error::BadParameters) => Self::BAD_PARAMETERS,
            Err(Error::MaxSessionsExceeded) => Self::MAX_SESSIONS_EXCEEDED,
            Err(Error::MaxRrRetryReached) => Self::MAX_RR_RETRY_REACHED,
            Err(Error::ProtocolSpecific) => Self::PROTOCOL_SPECIFIC,
            Err(Error::RemoteRequest) => Self::REMOTE_REQUEST,
            Err(Error::Timeout) => Self::TIMEOUT,
            Err(Error::CommandRetry) => Self::COMMAND_RETRY,
            Err(Error::DuplicatedSessionId) => Self::DUPLICATED_SESSION_ID,
            Err(_) => Self::UNKNOWN,
        }
    }
}

impl From<ShortAddressTwoWayRangingMeasurement> for ProtoRangingMeasurement {
    fn from(item: ShortAddressTwoWayRangingMeasurement) -> Self {
        let mut result = Self::new();
        result.set_mac_address(item.mac_address.into());
        result.set_status(item.status.into());
        result.set_nlos(item.nlos.into());
        result.set_distance(item.distance.into());
        result.set_aoa_azimuth(item.aoa_azimuth.into());
        result.set_aoa_azimuth_fom(item.aoa_azimuth_fom.into());
        result.set_aoa_elevation(item.aoa_elevation.into());
        result.set_aoa_elevation_fom(item.aoa_elevation_fom.into());
        result.set_aoa_destination_azimuth(item.aoa_destination_azimuth.into());
        result.set_aoa_destination_azimuth_fom(item.aoa_destination_azimuth_fom.into());
        result.set_aoa_destination_elevation(item.aoa_destination_elevation.into());
        result.set_aoa_destination_elevation_fom(item.aoa_destination_elevation_fom.into());
        result.set_slot_index(item.slot_index.into());
        result.set_rssi(item.rssi.into());
        result
    }
}

impl From<ExtendedAddressTwoWayRangingMeasurement> for ProtoRangingMeasurement {
    fn from(item: ExtendedAddressTwoWayRangingMeasurement) -> Self {
        let mut result = Self::new();
        result.set_mac_address(item.mac_address);
        result.set_status(item.status.into());
        result.set_nlos(item.nlos.into());
        result.set_distance(item.distance.into());
        result.set_aoa_azimuth(item.aoa_azimuth.into());
        result.set_aoa_azimuth_fom(item.aoa_azimuth_fom.into());
        result.set_aoa_elevation(item.aoa_elevation.into());
        result.set_aoa_elevation_fom(item.aoa_elevation_fom.into());
        result.set_aoa_destination_azimuth(item.aoa_destination_azimuth.into());
        result.set_aoa_destination_azimuth_fom(item.aoa_destination_azimuth_fom.into());
        result.set_aoa_destination_elevation(item.aoa_destination_elevation.into());
        result.set_aoa_destination_elevation_fom(item.aoa_destination_elevation_fom.into());
        result.set_slot_index(item.slot_index.into());
        result.set_rssi(item.rssi.into());
        result
    }
}

impl From<SessionRangeData> for ProtoSessionRangeData {
    fn from(item: SessionRangeData) -> Self {
        let mut result = Self::new();
        result.set_sequence_number(item.sequence_number);
        result.set_session_id(item.session_id);
        result.set_current_ranging_interval_ms(item.current_ranging_interval_ms);
        result.set_ranging_measurement_type(item.ranging_measurement_type.into());
        result.set_ranging_measurements(RepeatedField::from_vec(to_proto_ranging_measurements(
            item.ranging_measurements,
        )));
        result
    }
}

fn to_proto_ranging_measurements(item: RangingMeasurements) -> Vec<ProtoRangingMeasurement> {
    match item {
        RangingMeasurements::Short(arr) => arr.into_iter().map(|item| item.into()).collect(),
        RangingMeasurements::Extended(arr) => arr.into_iter().map(|item| item.into()).collect(),
    }
}

impl From<ProtoRangingRoundControl> for RangingRoundControl {
    fn from(item: ProtoRangingRoundControl) -> Self {
        Self {
            ranging_result_report_message: item.ranging_result_report_message,
            control_message: item.control_message,
            measurement_report_message: item.measurement_report_message,
        }
    }
}

impl From<RangingRoundControl> for ProtoRangingRoundControl {
    fn from(item: RangingRoundControl) -> Self {
        let mut res = Self::new();
        res.set_ranging_result_report_message(item.ranging_result_report_message);
        res.set_control_message(item.control_message);
        res.set_measurement_report_message(item.measurement_report_message);
        res
    }
}

impl From<ProtoResultReportConfig> for ResultReportConfig {
    fn from(item: ProtoResultReportConfig) -> Self {
        Self {
            tof: item.tof,
            aoa_azimuth: item.aoa_azimuth,
            aoa_elevation: item.aoa_elevation,
            aoa_fom: item.aoa_fom,
        }
    }
}

impl From<ResultReportConfig> for ProtoResultReportConfig {
    fn from(item: ResultReportConfig) -> Self {
        let mut res = Self::new();
        res.set_tof(item.tof);
        res.set_aoa_azimuth(item.aoa_azimuth);
        res.set_aoa_elevation(item.aoa_elevation);
        res.set_aoa_fom(item.aoa_fom);
        res
    }
}

fn to_uwb_address(bytes: Vec<u8>, mode: ProtoMacAddressMode) -> Option<UwbAddress> {
    match mode {
        ProtoMacAddressMode::MAC_ADDRESS_2_BYTES
        | ProtoMacAddressMode::MAC_ADDRESS_8_BYTES_2_BYTES_HEADER => {
            Some(UwbAddress::Short(bytes.try_into().ok()?))
        }
        ProtoMacAddressMode::MAC_ADDRESS_8_BYTES => {
            Some(UwbAddress::Extended(bytes.try_into().ok()?))
        }
    }
}

impl TryFrom<ProtoControlee> for Controlee {
    type Error = String;
    fn try_from(item: ProtoControlee) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            short_address: item
                .short_address
                .try_into()
                .map_err(|_| "Failed to convert short_address")?,
            subsession_id: item.subsession_id,
        })
    }
}

impl From<PowerStats> for ProtoPowerStats {
    fn from(item: PowerStats) -> Self {
        let mut res = Self::new();
        res.set_status(item.status.into());
        res.set_idle_time_ms(item.idle_time_ms);
        res.set_tx_time_ms(item.tx_time_ms);
        res.set_rx_time_ms(item.rx_time_ms);
        res.set_total_wake_count(item.total_wake_count);
        res
    }
}

impl From<FiraAppConfigParams> for ProtoFiraAppConfigParams {
    fn from(item: FiraAppConfigParams) -> Self {
        let mut res = Self::new();
        res.set_device_type((*item.device_type()).into());
        res.set_ranging_round_usage((*item.ranging_round_usage()).into());
        res.set_sts_config((*item.sts_config()).into());
        res.set_multi_node_mode((*item.multi_node_mode()).into());
        res.set_channel_number((*item.channel_number()).into());
        res.set_device_mac_address(item.device_mac_address().clone().into());
        res.set_dst_mac_address(
            item.dst_mac_address()
                .clone()
                .into_iter()
                .map(|addr| addr.into())
                .collect::<Vec<_>>()
                .into(),
        );
        res.set_slot_duration_rstu((*item.slot_duration_rstu()).into());
        res.set_ranging_interval_ms(*item.ranging_interval_ms());
        res.set_mac_fcs_type((*item.mac_fcs_type()).into());
        res.set_ranging_round_control(item.ranging_round_control().clone().into());
        res.set_aoa_result_request((*item.aoa_result_request()).into());
        res.set_range_data_ntf_config((*item.range_data_ntf_config()).into());
        res.set_range_data_ntf_proximity_near_cm((*item.range_data_ntf_proximity_near_cm()).into());
        res.set_range_data_ntf_proximity_far_cm((*item.range_data_ntf_proximity_far_cm()).into());
        res.set_device_role((*item.device_role()).into());
        res.set_rframe_config((*item.rframe_config()).into());
        res.set_preamble_code_index((*item.preamble_code_index()).into());
        res.set_sfd_id((*item.sfd_id()).into());
        res.set_psdu_data_rate((*item.psdu_data_rate()).into());
        res.set_preamble_duration((*item.preamble_duration()).into());
        res.set_ranging_time_struct((*item.ranging_time_struct()).into());
        res.set_slots_per_rr((*item.slots_per_rr()).into());
        res.set_tx_adaptive_payload_power((*item.tx_adaptive_payload_power()).into());
        res.set_responder_slot_index((*item.responder_slot_index()).into());
        res.set_prf_mode((*item.prf_mode()).into());
        res.set_scheduled_mode((*item.scheduled_mode()).into());
        res.set_key_rotation((*item.key_rotation()).into());
        res.set_key_rotation_rate((*item.key_rotation_rate()).into());
        res.set_session_priority((*item.session_priority()).into());
        res.set_mac_address_mode((*item.mac_address_mode()).into());
        res.set_vendor_id((*item.vendor_id()).into());
        res.set_static_sts_iv((*item.static_sts_iv()).into());
        res.set_number_of_sts_segments((*item.number_of_sts_segments()).into());
        res.set_max_rr_retry((*item.max_rr_retry()).into());
        res.set_uwb_initiation_time_ms(*item.uwb_initiation_time_ms());
        res.set_hopping_mode((*item.hopping_mode()).into());
        res.set_block_stride_length((*item.block_stride_length()).into());
        res.set_result_report_config(item.result_report_config().clone().into());
        res.set_in_band_termination_attempt_count(
            (*item.in_band_termination_attempt_count()).into(),
        );
        res.set_sub_session_id(*item.sub_session_id());
        res.set_bprf_phr_data_rate((*item.bprf_phr_data_rate()).into());
        res.set_max_number_of_measurements((*item.max_number_of_measurements()).into());
        res.set_sts_length((*item.sts_length()).into());
        res.set_number_of_range_measurements((*item.number_of_range_measurements()).into());
        res.set_number_of_aoa_azimuth_measurements(
            (*item.number_of_aoa_azimuth_measurements()).into(),
        );
        res.set_number_of_aoa_elevation_measurements(
            (*item.number_of_aoa_elevation_measurements()).into(),
        );

        res
    }
}

impl TryFrom<ProtoFiraAppConfigParams> for AppConfigParams {
    type Error = String;
    fn try_from(mut item: ProtoFiraAppConfigParams) -> std::result::Result<Self, Self::Error> {
        let device_mac_address =
            to_uwb_address(item.device_mac_address.clone(), item.mac_address_mode)
                .ok_or("Failed to convert device_mac_address")?;
        let mut dst_mac_address = vec![];
        for addr in item.dst_mac_address.clone().into_iter() {
            let addr = to_uwb_address(addr, item.mac_address_mode)
                .ok_or("Failed to convert dst_mac_address")?;
            dst_mac_address.push(addr);
        }

        let mut builder = FiraAppConfigParamsBuilder::new();
        builder
            .device_type(item.device_type.into())
            .ranging_round_usage(item.ranging_round_usage.into())
            .sts_config(item.sts_config.into())
            .multi_node_mode(item.multi_node_mode.into())
            .channel_number(item.channel_number.into())
            .device_mac_address(device_mac_address)
            .dst_mac_address(dst_mac_address)
            .slot_duration_rstu(
                item.slot_duration_rstu
                    .try_into()
                    .map_err(|_| "Failed to convert slot_duration_rstu")?,
            )
            .ranging_interval_ms(item.ranging_interval_ms)
            .mac_fcs_type(item.mac_fcs_type.into())
            .ranging_round_control(
                item.ranging_round_control.take().ok_or("ranging_round_control is empty")?.into(),
            )
            .aoa_result_request(item.aoa_result_request.into())
            .range_data_ntf_config(item.range_data_ntf_config.into())
            .range_data_ntf_proximity_near_cm(
                item.range_data_ntf_proximity_near_cm
                    .try_into()
                    .map_err(|_| "Failed to convert range_data_ntf_proximity_near_cm")?,
            )
            .range_data_ntf_proximity_far_cm(
                item.range_data_ntf_proximity_far_cm
                    .try_into()
                    .map_err(|_| "Failed to convert range_data_ntf_proximity_far_cm")?,
            )
            .device_role(item.device_role.into())
            .rframe_config(item.rframe_config.into())
            .preamble_code_index(
                item.preamble_code_index
                    .try_into()
                    .map_err(|_| "Failed to convert preamble_code_index")?,
            )
            .sfd_id(item.sfd_id.try_into().map_err(|_| "Failed to convert sfd_id")?)
            .psdu_data_rate(item.psdu_data_rate.into())
            .preamble_duration(item.preamble_duration.into())
            .ranging_time_struct(item.ranging_time_struct.into())
            .slots_per_rr(
                item.slots_per_rr.try_into().map_err(|_| "Failed to convert slots_per_rr")?,
            )
            .tx_adaptive_payload_power(item.tx_adaptive_payload_power.into())
            .responder_slot_index(
                item.responder_slot_index
                    .try_into()
                    .map_err(|_| "Failed to convert responder_slot_index")?,
            )
            .prf_mode(item.prf_mode.into())
            .scheduled_mode(item.scheduled_mode.into())
            .key_rotation(item.key_rotation.into())
            .key_rotation_rate(
                item.key_rotation_rate
                    .try_into()
                    .map_err(|_| "Failed to convert key_rotation_rate")?,
            )
            .session_priority(
                item.session_priority
                    .try_into()
                    .map_err(|_| "Failed to convert session_priority")?,
            )
            .mac_address_mode(item.mac_address_mode.into())
            .vendor_id(
                item.vendor_id.clone().try_into().map_err(|_| "Failed to convert vendor_id")?,
            )
            .static_sts_iv(
                item.static_sts_iv
                    .clone()
                    .try_into()
                    .map_err(|_| "Failed to convert static_sts_iv")?,
            )
            .number_of_sts_segments(
                item.number_of_sts_segments
                    .try_into()
                    .map_err(|_| "Failed to convert number_of_sts_segments")?,
            )
            .max_rr_retry(
                item.max_rr_retry.try_into().map_err(|_| "Failed to convert max_rr_retry")?,
            )
            .uwb_initiation_time_ms(item.uwb_initiation_time_ms)
            .hopping_mode(item.hopping_mode.into())
            .block_stride_length(
                item.block_stride_length
                    .try_into()
                    .map_err(|_| "Failed to convert block_stride_length")?,
            )
            .result_report_config(
                item.result_report_config.take().ok_or("ranging_round_control is empty")?.into(),
            )
            .in_band_termination_attempt_count(
                item.in_band_termination_attempt_count
                    .try_into()
                    .map_err(|_| "Failed to convert in_band_termination_attempt_count")?,
            )
            .sub_session_id(item.sub_session_id)
            .bprf_phr_data_rate(item.bprf_phr_data_rate.into())
            .max_number_of_measurements(
                item.max_number_of_measurements
                    .try_into()
                    .map_err(|_| "Failed to convert max_number_of_measurements")?,
            )
            .sts_length(item.sts_length.into())
            .number_of_range_measurements(
                item.number_of_range_measurements
                    .try_into()
                    .map_err(|_| "Failed to convert number_of_range_measurements")?,
            )
            .number_of_aoa_azimuth_measurements(
                item.number_of_aoa_azimuth_measurements
                    .try_into()
                    .map_err(|_| "Failed to convert number_of_aoa_azimuth_measurements")?,
            )
            .number_of_aoa_elevation_measurements(
                item.number_of_aoa_elevation_measurements
                    .try_into()
                    .map_err(|_| "Failed to convert number_of_aoa_elevation_measurements")?,
            );

        Ok(builder.build().ok_or("Failed to build FiraAppConfigParam from builder")?)
    }
}

impl Drop for ProtoFiraAppConfigParams {
    fn drop(&mut self) {
        // Zero out the sensitive data before releasing memory.
        self.vendor_id.zeroize();
        self.static_sts_iv.zeroize();
        self.sub_session_id.zeroize();
    }
}