#[cfg(test)]
mod tests {
    use crate::v2_0_1::datatypes::ACChargingParametersType;
    use crate::v2_0_1::datatypes::APNType;
    use crate::v2_0_1::datatypes::AdditionalInfoType;
    use crate::v2_0_1::datatypes::AuthorizationData;
    use crate::v2_0_1::datatypes::CertificateHashDataChainType;
    use crate::v2_0_1::datatypes::CertificateHashDataType;
    use crate::v2_0_1::datatypes::ChargingLimitType;
    use crate::v2_0_1::datatypes::ChargingNeedsType;
    use crate::v2_0_1::datatypes::ChargingProfileCriterionType;
    use crate::v2_0_1::datatypes::ChargingProfileType;
    use crate::v2_0_1::datatypes::ChargingSchedulePeriodType;
    use crate::v2_0_1::datatypes::ChargingScheduleType;
    use crate::v2_0_1::datatypes::ChargingStationType;
    use crate::v2_0_1::datatypes::ClearChargingProfileType;
    use crate::v2_0_1::datatypes::ClearMonitoringResultType;
    use crate::v2_0_1::datatypes::ComponentType;
    use crate::v2_0_1::datatypes::ComponentVariableType;
    use crate::v2_0_1::datatypes::CompositeScheduleType;
    use crate::v2_0_1::datatypes::ConsumptionCostType;
    use crate::v2_0_1::datatypes::CostType;
    use crate::v2_0_1::datatypes::DCChargingParametersType;
    use crate::v2_0_1::datatypes::EVSEType;
    use crate::v2_0_1::datatypes::EventDataType;
    use crate::v2_0_1::datatypes::FirmwareType;
    use crate::v2_0_1::datatypes::GetVariableDataType;
    use crate::v2_0_1::datatypes::GetVariableResultType;
    use crate::v2_0_1::datatypes::IdTokenInfoType;
    use crate::v2_0_1::datatypes::IdTokenType;
    use crate::v2_0_1::datatypes::LogParametersType;
    use crate::v2_0_1::datatypes::MessageContentType;
    use crate::v2_0_1::datatypes::MessageInfoType;
    use crate::v2_0_1::datatypes::MeterValueType;
    use crate::v2_0_1::datatypes::ModemType;
    use crate::v2_0_1::datatypes::MonitoringDataType;
    use crate::v2_0_1::datatypes::NetworkConnectionProfileType;
    use crate::v2_0_1::datatypes::OCSPRequestDataType;
    use crate::v2_0_1::datatypes::RelativeTimeIntervalType;
    use crate::v2_0_1::datatypes::ReportDataType;
    use crate::v2_0_1::datatypes::SalesTariffEntryType;
    use crate::v2_0_1::datatypes::SalesTariffType;
    use crate::v2_0_1::datatypes::SampledValueType;
    use crate::v2_0_1::datatypes::SetMonitoringDataType;
    use crate::v2_0_1::datatypes::SetMonitoringResultType;
    use crate::v2_0_1::datatypes::SetVariableDataType;
    use crate::v2_0_1::datatypes::SetVariableResultType;
    use crate::v2_0_1::datatypes::SignedMeterValueType;
    use crate::v2_0_1::datatypes::StatusInfoType;
    use crate::v2_0_1::datatypes::TransactionType;
    use crate::v2_0_1::datatypes::UnitOfMeasureType;
    use crate::v2_0_1::datatypes::VPNType;
    use crate::v2_0_1::datatypes::VariableAttributeType;
    use crate::v2_0_1::datatypes::VariableCharacteristicsType;
    use crate::v2_0_1::datatypes::VariableMonitoringType;
    use crate::v2_0_1::datatypes::VariableType;
    use crate::v2_0_1::enumerations::APNAuthenticationEnumType;
    use crate::v2_0_1::enumerations::AttributeEnumType;
    use crate::v2_0_1::enumerations::AuthorizationStatusEnumType;
    use crate::v2_0_1::enumerations::AuthorizeCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::BootReasonEnumType;
    use crate::v2_0_1::enumerations::CancelReservationStatusEnumType;
    use crate::v2_0_1::enumerations::CertificateActionEnumType;
    use crate::v2_0_1::enumerations::CertificateSignedStatusEnumType;
    use crate::v2_0_1::enumerations::CertificateSigningUseEnumType;
    use crate::v2_0_1::enumerations::ChangeAvailabilityStatusEnumType;
    use crate::v2_0_1::enumerations::ChargingLimitSourceEnumType;
    use crate::v2_0_1::enumerations::ChargingProfileKindEnumType;
    use crate::v2_0_1::enumerations::ChargingProfilePurposeEnumType;
    use crate::v2_0_1::enumerations::ChargingProfileStatusEnumType;
    use crate::v2_0_1::enumerations::ChargingRateUnitEnumType;
    use crate::v2_0_1::enumerations::ChargingStateEnumType;
    use crate::v2_0_1::enumerations::ClearCacheStatusEnumType;
    use crate::v2_0_1::enumerations::ClearChargingProfileStatusEnumType;
    use crate::v2_0_1::enumerations::ClearMessageStatusEnumType;
    use crate::v2_0_1::enumerations::ClearMonitoringStatusEnumType;
    use crate::v2_0_1::enumerations::ComponentCriterionEnumType;
    use crate::v2_0_1::enumerations::ConnectorEnumType;
    use crate::v2_0_1::enumerations::ConnectorStatusEnumType;
    use crate::v2_0_1::enumerations::CostKindEnumType;
    use crate::v2_0_1::enumerations::CustomerInformationStatusEnumType;
    use crate::v2_0_1::enumerations::DataEnumType;
    use crate::v2_0_1::enumerations::DataTransferStatusEnumType;
    use crate::v2_0_1::enumerations::DeleteCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::DisplayMessageStatusEnumType;
    use crate::v2_0_1::enumerations::EnergyTransferModeEnumType;
    use crate::v2_0_1::enumerations::EventNotificationEnumType;
    use crate::v2_0_1::enumerations::EventTriggerEnumType;
    use crate::v2_0_1::enumerations::FirmwareStatusEnumType;
    use crate::v2_0_1::enumerations::GenericDeviceModelStatusEnumType;
    use crate::v2_0_1::enumerations::GenericStatusEnumType;
    use crate::v2_0_1::enumerations::GetCertificateIdUseEnumType;
    use crate::v2_0_1::enumerations::GetCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::GetChargingProfileStatusEnumType;
    use crate::v2_0_1::enumerations::GetDisplayMessagesStatusEnumType;
    use crate::v2_0_1::enumerations::GetInstalledCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::GetVariableStatusEnumType;
    use crate::v2_0_1::enumerations::HashAlgorithmEnumType;
    use crate::v2_0_1::enumerations::IdTokenEnumType;
    use crate::v2_0_1::enumerations::InstallCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::InstallCertificateUseEnumType;
    use crate::v2_0_1::enumerations::Iso15118EVCertificateStatusEnumType;
    use crate::v2_0_1::enumerations::LocationEnumType;
    use crate::v2_0_1::enumerations::LogEnumType;
    use crate::v2_0_1::enumerations::LogStatusEnumType;
    use crate::v2_0_1::enumerations::MeasurandEnumType;
    use crate::v2_0_1::enumerations::MessageFormatEnumType;
    use crate::v2_0_1::enumerations::MessagePriorityEnumType;
    use crate::v2_0_1::enumerations::MessageStateEnumType;
    use crate::v2_0_1::enumerations::MessageTriggerEnumType;
    use crate::v2_0_1::enumerations::MonitorEnumType;
    use crate::v2_0_1::enumerations::MonitoringBaseEnumType;
    use crate::v2_0_1::enumerations::MonitoringCriterionEnumType;
    use crate::v2_0_1::enumerations::MutabilityEnumType;
    use crate::v2_0_1::enumerations::NotifyEVChargingNeedsStatusEnumType;
    use crate::v2_0_1::enumerations::OCPPInterfaceEnumType;
    use crate::v2_0_1::enumerations::OCPPTransportEnumType;
    use crate::v2_0_1::enumerations::OCPPVersionEnumType;
    use crate::v2_0_1::enumerations::OperationalStatusEnumType;
    use crate::v2_0_1::enumerations::PhaseEnumType;
    use crate::v2_0_1::enumerations::PublishFirmwareStatusEnumType;
    use crate::v2_0_1::enumerations::ReadingContextEnumType;
    use crate::v2_0_1::enumerations::ReasonEnumType;
    use crate::v2_0_1::enumerations::RecurrencyKindEnumType;
    use crate::v2_0_1::enumerations::RegistrationStatusEnumType;
    use crate::v2_0_1::enumerations::ReportBaseEnumType;
    use crate::v2_0_1::enumerations::RequestStartStopStatusEnumType;
    use crate::v2_0_1::enumerations::ReservationUpdateStatusEnumType;
    use crate::v2_0_1::enumerations::ReserveNowStatusEnumType;
    use crate::v2_0_1::enumerations::ResetEnumType;
    use crate::v2_0_1::enumerations::ResetStatusEnumType;
    use crate::v2_0_1::enumerations::SendLocalListStatusEnumType;
    use crate::v2_0_1::enumerations::SetMonitoringStatusEnumType;
    use crate::v2_0_1::enumerations::SetNetworkProfileStatusEnumType;
    use crate::v2_0_1::enumerations::SetVariableStatusEnumType;
    use crate::v2_0_1::enumerations::TransactionEventEnumType;
    use crate::v2_0_1::enumerations::TriggerMessageStatusEnumType;
    use crate::v2_0_1::enumerations::TriggerReasonEnumType;
    use crate::v2_0_1::enumerations::UnlockStatusEnumType;
    use crate::v2_0_1::enumerations::UnpublishFirmwareStatusEnumType;
    use crate::v2_0_1::enumerations::UpdateEnumType;
    use crate::v2_0_1::enumerations::UpdateFirmwareStatusEnumType;
    use crate::v2_0_1::enumerations::UploadLogStatusEnumType;
    use crate::v2_0_1::enumerations::VPNEnumType;
    use crate::v2_0_1::messages::{AuthorizeRequest, AuthorizeResponse};
    use crate::v2_0_1::messages::{BootNotificationRequest, BootNotificationResponse};
    use crate::v2_0_1::messages::{CancelReservationRequest, CancelReservationResponse};
    use crate::v2_0_1::messages::{CertificateSignedRequest, CertificateSignedResponse};
    use crate::v2_0_1::messages::{ChangeAvailabilityRequest, ChangeAvailabilityResponse};
    use crate::v2_0_1::messages::{ClearCacheRequest, ClearCacheResponse};
    use crate::v2_0_1::messages::{ClearChargingProfileRequest, ClearChargingProfileResponse};
    use crate::v2_0_1::messages::{ClearDisplayMessageRequest, ClearDisplayMessageResponse};
    use crate::v2_0_1::messages::{
        ClearVariableMonitoringRequest, ClearVariableMonitoringResponse,
    };
    use crate::v2_0_1::messages::{ClearedChargingLimitRequest, ClearedChargingLimitResponse};
    use crate::v2_0_1::messages::{CostUpdatedRequest, CostUpdatedResponse};
    use crate::v2_0_1::messages::{CustomerInformationRequest, CustomerInformationResponse};
    use crate::v2_0_1::messages::{DataTransferRequest, DataTransferResponse};
    use crate::v2_0_1::messages::{DeleteCertificateRequest, DeleteCertificateResponse};
    use crate::v2_0_1::messages::{
        FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
    };
    use crate::v2_0_1::messages::{Get15118EVCertificateRequest, Get15118EVCertificateResponse};
    use crate::v2_0_1::messages::{GetBaseReportRequest, GetBaseReportResponse};
    use crate::v2_0_1::messages::{GetCertificateStatusRequest, GetCertificateStatusResponse};
    use crate::v2_0_1::messages::{GetChargingProfilesRequest, GetChargingProfilesResponse};
    use crate::v2_0_1::messages::{GetCompositeScheduleRequest, GetCompositeScheduleResponse};
    use crate::v2_0_1::messages::{GetDisplayMessagesRequest, GetDisplayMessagesResponse};
    use crate::v2_0_1::messages::{
        GetInstalledCertificateIdsRequest, GetInstalledCertificateIdsResponse,
    };
    use crate::v2_0_1::messages::{GetLocalListVersionRequest, GetLocalListVersionResponse};
    use crate::v2_0_1::messages::{GetLogRequest, GetLogResponse};
    use crate::v2_0_1::messages::{GetMonitoringReportRequest, GetMonitoringReportResponse};
    use crate::v2_0_1::messages::{GetReportRequest, GetReportResponse};
    use crate::v2_0_1::messages::{GetTransactionStatusRequest, GetTransactionStatusResponse};
    use crate::v2_0_1::messages::{GetVariablesRequest, GetVariablesResponse};
    use crate::v2_0_1::messages::{HeartbeatRequest, HeartbeatResponse};
    use crate::v2_0_1::messages::{InstallCertificateRequest, InstallCertificateResponse};
    use crate::v2_0_1::messages::{LogStatusNotificationRequest, LogStatusNotificationResponse};
    use crate::v2_0_1::messages::{MeterValuesRequest, MeterValuesResponse};
    use crate::v2_0_1::messages::{NotifyChargingLimitRequest, NotifyChargingLimitResponse};
    use crate::v2_0_1::messages::{
        NotifyCustomerInformationRequest, NotifyCustomerInformationResponse,
    };
    use crate::v2_0_1::messages::{NotifyDisplayMessagesRequest, NotifyDisplayMessagesResponse};
    use crate::v2_0_1::messages::{NotifyEVChargingNeedsRequest, NotifyEVChargingNeedsResponse};
    use crate::v2_0_1::messages::{
        NotifyEVChargingScheduleRequest, NotifyEVChargingScheduleResponse,
    };
    use crate::v2_0_1::messages::{NotifyEventRequest, NotifyEventResponse};
    use crate::v2_0_1::messages::{NotifyMonitoringReportRequest, NotifyMonitoringReportResponse};
    use crate::v2_0_1::messages::{NotifyReportRequest, NotifyReportResponse};
    use crate::v2_0_1::messages::{PublishFirmwareRequest, PublishFirmwareResponse};
    use crate::v2_0_1::messages::{
        PublishFirmwareStatusNotificationRequest, PublishFirmwareStatusNotificationResponse,
    };
    use crate::v2_0_1::messages::{ReportChargingProfilesRequest, ReportChargingProfilesResponse};
    use crate::v2_0_1::messages::{
        RequestStartTransactionRequest, RequestStartTransactionResponse,
    };
    use crate::v2_0_1::messages::{RequestStopTransactionRequest, RequestStopTransactionResponse};
    use crate::v2_0_1::messages::{
        ReservationStatusUpdateRequest, ReservationStatusUpdateResponse,
    };
    use crate::v2_0_1::messages::{ReserveNowRequest, ReserveNowResponse};
    use crate::v2_0_1::messages::{ResetRequest, ResetResponse};
    use crate::v2_0_1::messages::{
        SecurityEventNotificationRequest, SecurityEventNotificationResponse,
    };
    use crate::v2_0_1::messages::{SendLocalListRequest, SendLocalListResponse};
    use crate::v2_0_1::messages::{SetChargingProfileRequest, SetChargingProfileResponse};
    use crate::v2_0_1::messages::{SetDisplayMessageRequest, SetDisplayMessageResponse};
    use crate::v2_0_1::messages::{SetMonitoringBaseRequest, SetMonitoringBaseResponse};
    use crate::v2_0_1::messages::{SetMonitoringLevelRequest, SetMonitoringLevelResponse};
    use crate::v2_0_1::messages::{SetNetworkProfileRequest, SetNetworkProfileResponse};
    use crate::v2_0_1::messages::{SetVariableMonitoringRequest, SetVariableMonitoringResponse};
    use crate::v2_0_1::messages::{SetVariablesRequest, SetVariablesResponse};
    use crate::v2_0_1::messages::{SignCertificateRequest, SignCertificateResponse};
    use crate::v2_0_1::messages::{StatusNotificationRequest, StatusNotificationResponse};
    use crate::v2_0_1::messages::{TransactionEventRequest, TransactionEventResponse};
    use crate::v2_0_1::messages::{TriggerMessageRequest, TriggerMessageResponse};
    use crate::v2_0_1::messages::{UnlockConnectorRequest, UnlockConnectorResponse};
    use crate::v2_0_1::messages::{UnpublishFirmwareRequest, UnpublishFirmwareResponse};
    use crate::v2_0_1::messages::{UpdateFirmwareRequest, UpdateFirmwareResponse};
    use chrono::Utc;
    use jsonschema::Validator;
    use rust_decimal_macros::dec;

    #[test]
    fn validate_authorize_request() {
        let test = AuthorizeRequest {
            custom_data: None,
            certificate: Some("".to_string()),
            id_token: IdTokenType {
                custom_data: None,
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    custom_data: None,
                    additional_id_token: "more than 5 characters".to_string(),
                    kind: "".to_string(),
                }]),
            },
            iso_15118_certificate_hash_data: Some(vec![OCSPRequestDataType {
                custom_data: None,
                hash_algorithm: HashAlgorithmEnumType::SHA256,
                issuer_name_hash: "".to_string(),
                issuer_key_hash: "".to_string(),
                serial_number: "".to_string(),
                responder_url: "".to_string(),
            }]),
        };

        let schema = include_str!("schemas/v2.0.1/AuthorizeRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_authorize_response() {
        let test = AuthorizeResponse {
            custom_data: None,
            certificate_status: Some(AuthorizeCertificateStatusEnumType::Accepted),
            id_token_info: IdTokenInfoType {
                custom_data: None,
                status: AuthorizationStatusEnumType::Accepted,
                cache_expiry_date_time: Some(Utc::now()),
                charging_priority: Some(1),
                language1: Some("English".to_string()),
                evse_id: Some(vec![1]),
                language2: Some("Chinese".to_string()),
                group_id_token: Some(IdTokenType {
                    custom_data: None,
                    id_token: "".to_string(),
                    kind: IdTokenEnumType::Central,
                    additional_info: Some(vec![AdditionalInfoType {
                        custom_data: None,
                        additional_id_token: "".to_string(),
                        kind: "".to_string(),
                    }]),
                }),
                personal_message: Some(MessageContentType {
                    custom_data: None,
                    format: MessageFormatEnumType::ASCII,
                    language: Some("Swedish".to_string()),
                    content: "".to_string(),
                }),
            },
        };

        let schema = include_str!("schemas/v2.0.1/AuthorizeResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_boot_notification_request() {
        let test = BootNotificationRequest {
            custom_data: None,
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                custom_data: None,
                model: "SingleSocketCharger".to_string(),
                vendor_name: "VendorX".to_string(),
                serial_number: Some("serial_number".to_string()),
                firmware_version: Some("firmware_version".to_string()),
                modem: Some(ModemType {
                    custom_data: None,
                    iccid: Some("iccid".to_string()),
                    imsi: Some("imsi".to_string()),
                }),
            },
        };
        let schema = include_str!("schemas/v2.0.1/BootNotificationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_boot_notification_response() {
        let test = BootNotificationResponse {
            custom_data: None,
            current_time: Utc::now(),
            interval: 10,
            status: RegistrationStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("schemas/v2.0.1/BootNotificationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cancel_reservation_request() {
        let test = CancelReservationRequest {
            custom_data: None,
            reservation_id: 0,
        };
        let schema = include_str!("schemas/v2.0.1/CancelReservationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cancel_reservation_response() {
        let test = CancelReservationResponse {
            custom_data: None,
            status: CancelReservationStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("schemas/v2.0.1/CancelReservationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_certificate_signed_request() {
        let test = CertificateSignedRequest {
            custom_data: None,
            certificate_chain: "certificate_chain".to_string(),
            certificate_type: Some(CertificateSigningUseEnumType::ChargingStationCertificate),
        };
        let schema = include_str!("schemas/v2.0.1/CertificateSignedRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_certificate_signed_response() {
        let test = CertificateSignedResponse {
            custom_data: None,
            status: CertificateSignedStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("schemas/v2.0.1/CertificateSignedResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_change_availability_request() {
        let test = ChangeAvailabilityRequest {
            custom_data: None,
            operational_status: OperationalStatusEnumType::Inoperative,
            evse: Some(EVSEType {
                custom_data: None,
                id: 0,
                connector_id: Some(1),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/ChangeAvailabilityRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_change_availability_response() {
        let test = ChangeAvailabilityResponse {
            custom_data: None,
            status: ChangeAvailabilityStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("schemas/v2.0.1/ChangeAvailabilityResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_cache_request() {
        let test = ClearCacheRequest { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/ClearCacheRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_cache_response() {
        let test = ClearCacheResponse {
            custom_data: None,
            status: ClearCacheStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("schemas/v2.0.1/ClearCacheResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_charging_profile_request() {
        let test = ClearChargingProfileRequest {
            custom_data: None,
            charging_profile_id: Some(1),
            charging_profile_criteria: Some(ClearChargingProfileType {
                custom_data: None,
                evse_id: Some(1),
                charging_profile_purpose: Some(
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                ),
                stack_level: Some(1),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/ClearChargingProfileRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_charging_profile_response() {
        let test = ClearChargingProfileResponse {
            custom_data: None,
            status: ClearChargingProfileStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("schemas/v2.0.1/ClearChargingProfileResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_display_message_request() {
        let test = ClearDisplayMessageRequest {
            custom_data: None,
            id: 0,
        };
        let schema = include_str!("schemas/v2.0.1/ClearDisplayMessageRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_display_message_response() {
        let test = ClearDisplayMessageResponse {
            custom_data: None,
            status: ClearMessageStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "reason_code".to_string(),
                additional_info: Some("additional_info".to_string()),
            }),
        };

        let schema = include_str!("schemas/v2.0.1/ClearDisplayMessageResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cleared_charging_limit_request() {
        let test = ClearedChargingLimitRequest {
            custom_data: None,
            charging_limit_source: ChargingLimitSourceEnumType::EMS,
            evse_id: Some(1),
        };
        let schema = include_str!("schemas/v2.0.1/ClearedChargingLimitRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cleared_charging_limit_response() {
        let test = ClearedChargingLimitResponse { custom_data: None };

        let schema = include_str!("schemas/v2.0.1/ClearedChargingLimitResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_variable_monitoring_request() {
        let test = ClearVariableMonitoringRequest {
            custom_data: None,
            id: vec![0],
        };
        let schema = include_str!("schemas/v2.0.1/ClearVariableMonitoringRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_clear_variable_monitoring_response() {
        let test = ClearVariableMonitoringResponse {
            custom_data: None,
            clear_monitoring_result: vec![ClearMonitoringResultType {
                custom_data: None,
                status: ClearMonitoringStatusEnumType::Accepted,
                id: 0,
                status_info: Some(StatusInfoType {
                    custom_data: None,
                    reason_code: "".to_string(),
                    additional_info: Some("".to_string()),
                }),
            }],
        };
        let schema = include_str!("schemas/v2.0.1/ClearVariableMonitoringResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }

    #[test]
    fn validate_cost_updated_request() {
        let test = CostUpdatedRequest {
            custom_data: None,
            total_cost: dec!(0.0),
            transaction_id: "".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/CostUpdatedRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_cost_updated_response() {
        let test = CostUpdatedResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/CostUpdatedResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_customer_information_request() {
        let test = CustomerInformationRequest {
            custom_data: None,
            request_id: 0,
            report: false,
            clear: false,
            customer_identifier: Some("customer_identifier".to_string()),
            id_token: Some(IdTokenType {
                custom_data: None,
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    custom_data: None,
                    additional_id_token: "additional_id_token".to_string(),
                    kind: "type".to_string(),
                }]),
            }),
            customer_certificate: Some(CertificateHashDataType {
                custom_data: None,
                hash_algorithm: HashAlgorithmEnumType::SHA256,
                issuer_name_hash: "issuer_name".to_string(),
                issuer_key_hash: "issuer_key_hash".to_string(),
                serial_number: "serial_number".to_string(),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/CustomerInformationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_customer_information_response() {
        let test = CustomerInformationResponse {
            custom_data: None,
            status: CustomerInformationStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/CustomerInformationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_data_transfer_request_no_data() {
        let test = DataTransferRequest {
            custom_data: None,
            message_id: Some("message_id".to_string()),
            data: None,
            vendor_id: "vendor_id".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/DataTransferRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_data_transfer_request() {
        let test = DataTransferRequest {
            custom_data: None,
            message_id: Some("message_id".to_string()),
            data: Some("data".to_string()),
            vendor_id: "vendor_id".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/DataTransferRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_data_transfer_response() {
        let test = DataTransferResponse {
            custom_data: None,
            status: DataTransferStatusEnumType::Accepted,
            data: None,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/DataTransferResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_delete_certificate_request() {
        let test = DeleteCertificateRequest {
            custom_data: None,
            certificate_hash_data: CertificateHashDataType {
                custom_data: None,
                hash_algorithm: HashAlgorithmEnumType::SHA256,
                issuer_name_hash: "".to_string(),
                issuer_key_hash: "".to_string(),
                serial_number: "".to_string(),
            },
        };
        let schema = include_str!("schemas/v2.0.1/DeleteCertificateRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_delete_certificate_response() {
        let test = DeleteCertificateResponse {
            custom_data: None,
            status: DeleteCertificateStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/DeleteCertificateResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_firmware_status_notification_request() {
        let test = FirmwareStatusNotificationRequest {
            custom_data: None,
            status: FirmwareStatusEnumType::Downloaded,
            request_id: Some(1),
        };
        let schema = include_str!("schemas/v2.0.1/FirmwareStatusNotificationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_firmware_status_notification_response() {
        let test = FirmwareStatusNotificationResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/FirmwareStatusNotificationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get15118ev_certificate_request() {
        let test = Get15118EVCertificateRequest {
            custom_data: None,
            iso_15118_schema_version: "".to_string(),
            action: CertificateActionEnumType::Install,
            exi_request: "".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/Get15118EVCertificateRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get15118ev_certificate_response() {
        let test = Get15118EVCertificateResponse {
            custom_data: None,
            status: Iso15118EVCertificateStatusEnumType::Accepted,
            exi_response: "".to_string(),
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/Get15118EVCertificateResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_base_report_request() {
        let test = GetBaseReportRequest {
            custom_data: None,
            request_id: 0,
            report_base: ReportBaseEnumType::ConfigurationInventory,
        };
        let schema = include_str!("schemas/v2.0.1/GetBaseReportRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_base_report_response() {
        let test = GetBaseReportResponse {
            custom_data: None,
            status: GenericDeviceModelStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetBaseReportResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_certificate_status_request() {
        let test = GetCertificateStatusRequest {
            custom_data: None,
            ocsp_request_data: OCSPRequestDataType {
                custom_data: None,
                hash_algorithm: HashAlgorithmEnumType::SHA256,
                issuer_name_hash: "".to_string(),
                issuer_key_hash: "".to_string(),
                serial_number: "".to_string(),
                responder_url: "".to_string(),
            },
        };
        let schema = include_str!("schemas/v2.0.1/GetCertificateStatusRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_certificate_status_response() {
        let test = GetCertificateStatusResponse {
            custom_data: None,
            status: GetCertificateStatusEnumType::Accepted,
            ocsp_result: Some("ocsp_result".to_string()),
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetCertificateStatusResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_charging_profiles_request() {
        let test = GetChargingProfilesRequest {
            custom_data: None,
            request_id: 0,
            evse_id: Some(1),
            charging_profile: ChargingProfileCriterionType {
                custom_data: None,
                charging_profile_purpose: Some(
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                ),
                stack_level: Some(1),
                charging_profile_id: Some(vec![1]),
                charging_limit_source: Some(vec![ChargingLimitSourceEnumType::CSO]),
            },
        };
        let schema = include_str!("schemas/v2.0.1/GetChargingProfilesRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_charging_profiles_response() {
        let test = GetChargingProfilesResponse {
            custom_data: None,
            status: GetChargingProfileStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetChargingProfilesResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_composite_schedule_request() {
        let test = GetCompositeScheduleRequest {
            custom_data: None,
            duration: 0,
            charging_rate_unit: Some(ChargingRateUnitEnumType::W),
            evse_id: 0,
        };
        let schema = include_str!("schemas/v2.0.1/GetCompositeScheduleRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_composite_schedule_response() {
        let test = GetCompositeScheduleResponse {
            custom_data: None,
            status: GenericStatusEnumType::Accepted,
            schedule: Some(CompositeScheduleType {
                custom_data: None,
                evse_id: 0,
                duration: 0,
                schedule_start: Utc::now(),
                charging_rate_unit: ChargingRateUnitEnumType::W,
                charging_schedule_period: vec![ChargingSchedulePeriodType {
                    custom_data: None,
                    start_period: 0,
                    limit: dec!(0.0),
                    number_phases: Some(1),
                    phase_to_use: Some(1),
                }],
            }),
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetCompositeScheduleResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_display_messages_request() {
        let test = GetDisplayMessagesRequest {
            custom_data: None,
            id: Some(vec![1]),
            request_id: 0,
            priority: Some(MessagePriorityEnumType::AlwaysFront),
            state: Some(MessageStateEnumType::Charging),
        };
        let schema = include_str!("schemas/v2.0.1/GetDisplayMessagesRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_display_messages_response() {
        let test = GetDisplayMessagesResponse {
            custom_data: None,
            status: GetDisplayMessagesStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetDisplayMessagesResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_installed_certificate_ids_request() {
        let test = GetInstalledCertificateIdsRequest {
            custom_data: None,
            certificate_type: Some(vec![GetCertificateIdUseEnumType::CSMSRootCertificate]),
        };
        let schema = include_str!("schemas/v2.0.1/GetInstalledCertificateIdsRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_installed_certificate_ids_response() {
        let test = GetInstalledCertificateIdsResponse {
            custom_data: None,
            status: GetInstalledCertificateStatusEnumType::Accepted,
            certificate_hash_data_chain: Some(vec![CertificateHashDataChainType {
                custom_data: None,
                certificate_type: GetCertificateIdUseEnumType::V2GRootCertificate,
                certificate_hash_data: CertificateHashDataType {
                    custom_data: None,
                    hash_algorithm: HashAlgorithmEnumType::SHA256,
                    issuer_name_hash: "issuer_name_hash".to_string(),
                    issuer_key_hash: "issuer_key_hash".to_string(),
                    serial_number: "serial_number".to_string(),
                },
                child_certificate_hash_data: Some(vec![CertificateHashDataType {
                    custom_data: None,
                    hash_algorithm: HashAlgorithmEnumType::SHA256,
                    issuer_name_hash: "issuer_name_hash".to_string(),
                    issuer_key_hash: "issuer_key_hash".to_string(),
                    serial_number: "serial_number".to_string(),
                }]),
            }]),
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetInstalledCertificateIdsResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_local_list_version_request() {
        let test = GetLocalListVersionRequest { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/GetLocalListVersionRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_local_list_version_response() {
        let test = GetLocalListVersionResponse {
            custom_data: None,
            version_number: 0,
        };
        let schema = include_str!("schemas/v2.0.1/GetLocalListVersionResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_log_request() {
        let test = GetLogRequest {
            custom_data: None,
            log_type: LogEnumType::DiagnosticsLog,
            request_id: 0,
            retries: Some(1),
            retry_interval: Some(1),
            log: LogParametersType {
                custom_data: None,
                remote_location: "remote_location".to_string(),
                oldest_timestamp: Some(Utc::now()),
                latest_timestamp: Some(Utc::now()),
            },
        };
        let schema = include_str!("schemas/v2.0.1/GetLogRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_log_response() {
        let test = GetLogResponse {
            custom_data: None,
            status: LogStatusEnumType::Accepted,
            filename: Some("/filename".to_string()),
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetLogResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_monitoring_report_request() {
        let test = GetMonitoringReportRequest {
            custom_data: None,
            request_id: 0,
            monitoring_criteria: Some(vec![MonitoringCriterionEnumType::DeltaMonitoring]),
            component_variable: Some(vec![ComponentVariableType {
                custom_data: None,
                component: ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(1),
                    }),
                },
                variable: Some(VariableType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("instance".to_string()),
                }),
            }]),
        };
        let schema = include_str!("schemas/v2.0.1/GetMonitoringReportRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_monitoring_report_response() {
        let test = GetMonitoringReportResponse {
            custom_data: None,
            status: GenericDeviceModelStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetMonitoringReportResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_report_request() {
        let test = GetReportRequest {
            custom_data: None,
            request_id: 0,
            component_criteria: Some(vec![ComponentCriterionEnumType::Active]),
            component_variable: Some(vec![ComponentVariableType {
                custom_data: None,
                component: ComponentType {
                    custom_data: None,
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(1),
                    }),
                },
                variable: Some(VariableType {
                    custom_data: None,
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                }),
            }]),
        };
        let schema = include_str!("schemas/v2.0.1/GetReportRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_report_response() {
        let test = GetReportResponse {
            custom_data: None,
            status: GenericDeviceModelStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/GetReportResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_transaction_status_request() {
        let test = GetTransactionStatusRequest {
            custom_data: None,
            transaction_id: Some("transaction_id".to_string()),
        };
        let schema = include_str!("schemas/v2.0.1/GetTransactionStatusRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_transaction_status_response() {
        let test = GetTransactionStatusResponse {
            custom_data: None,
            ongoing_indicator: Some(true),
            messages_in_queue: false,
        };
        let schema = include_str!("schemas/v2.0.1/GetTransactionStatusResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_variables_request() {
        let test = GetVariablesRequest {
            custom_data: None,
            get_variable_data: vec![GetVariableDataType {
                custom_data: None,
                attribute_type: Some(AttributeEnumType::MaxSet),
                component: ComponentType {
                    custom_data: None,
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(1),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                },
            }],
        };
        let schema = include_str!("schemas/v2.0.1/GetVariablesRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_get_variables_response() {
        let test = GetVariablesResponse {
            custom_data: None,
            get_variable_result: vec![GetVariableResultType {
                custom_data: None,
                attribute_status: GetVariableStatusEnumType::Accepted,
                attribute_type: Some(AttributeEnumType::Actual),
                attribute_value: Some("attribute_value".to_string()),
                component: ComponentType {
                    custom_data: None,
                    name: "name".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
                attribute_status_info: Some(StatusInfoType {
                    custom_data: None,
                    reason_code: "".to_string(),
                    additional_info: Some("".to_string()),
                }),
            }],
        };
        let schema = include_str!("schemas/v2.0.1/GetVariablesResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_heartbeat_request() {
        let test = HeartbeatRequest { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/HeartbeatRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_heartbeat_response() {
        let test = HeartbeatResponse {
            custom_data: None,
            current_time: Utc::now(),
        };
        let schema = include_str!("schemas/v2.0.1/HeartbeatResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_install_certificate_request() {
        let test = InstallCertificateRequest {
            custom_data: None,
            certificate_type: InstallCertificateUseEnumType::V2GRootCertificate,
            certificate: "".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/InstallCertificateRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_install_certificate_response() {
        let test = InstallCertificateResponse {
            custom_data: None,
            status: InstallCertificateStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/InstallCertificateResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_log_status_notification_request() {
        let test = LogStatusNotificationRequest {
            custom_data: None,
            status: UploadLogStatusEnumType::BadMessage,
            request_id: Some(1),
        };
        let schema = include_str!("schemas/v2.0.1/LogStatusNotificationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_log_status_notification_response() {
        let test = LogStatusNotificationResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/LogStatusNotificationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_meter_values_request() {
        let test = MeterValuesRequest {
            custom_data: None,
            evse_id: 0,
            meter_value: vec![MeterValueType {
                custom_data: None,
                timestamp: Utc::now(),
                sampled_value: vec![SampledValueType {
                    custom_data: None,
                    value: dec!(0.0),
                    context: Some(ReadingContextEnumType::SampleClock),
                    measurand: Some(MeasurandEnumType::CurrentExport),
                    phase: Some(PhaseEnumType::L1),
                    location: Some(LocationEnumType::Body),
                    signed_meter_value: Some(SignedMeterValueType {
                        custom_data: None,
                        signed_meter_data: "signed_meter_data".to_string(),
                        signing_method: "signing_method".to_string(),
                        encoding_method: "encoding_method".to_string(),
                        public_key: "public_key".to_string(),
                    }),
                    unit_of_measure: Some(UnitOfMeasureType {
                        custom_data: None,
                        unit: Some("unit".to_string()),
                        multiplier: Some(1),
                    }),
                }],
            }],
        };
        let schema = include_str!("schemas/v2.0.1/MeterValuesRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_meter_values_response() {
        let test = MeterValuesResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/MeterValuesResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_charging_limit_request() {
        let test = NotifyChargingLimitRequest {
            custom_data: None,
            evse_id: Some(0),
            charging_limit: ChargingLimitType {
                custom_data: None,
                charging_limit_source: ChargingLimitSourceEnumType::EMS,
                is_grid_critical: Some(false),
            },
            charging_schedule: Some(vec![ChargingScheduleType {
                custom_data: None,
                id: 0,
                start_schedule: Some(Utc::now()),
                duration: Some(10),
                charging_rate_unit: ChargingRateUnitEnumType::W,
                min_charging_rate: Some(dec!(10.0)),
                charging_schedule_period: vec![ChargingSchedulePeriodType {
                    custom_data: None,
                    start_period: 0,
                    limit: dec!(0.0),
                    number_phases: Some(0),
                    phase_to_use: Some(0),
                }],
                sales_tariff: Some(SalesTariffType {
                    custom_data: None,
                    id: 0,
                    sales_tariff_description: Some("sales_tariff_description".to_string()),
                    num_e_price_levels: Some(0),
                    sales_tariff_entry: vec![SalesTariffEntryType {
                        custom_data: None,
                        e_price_level: Some(0),
                        relative_time_interval: RelativeTimeIntervalType {
                            custom_data: None,
                            start: 0,
                            duration: Some(0),
                        },
                        consumption_cost: Some(vec![ConsumptionCostType {
                            custom_data: None,
                            start_value: dec!(0.0),
                            cost: vec![CostType {
                                custom_data: None,
                                cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                amount: 0,
                                amount_multiplier: Some(0),
                            }],
                        }]),
                    }],
                }),
            }]),
        };
        let schema = include_str!("schemas/v2.0.1/NotifyChargingLimitRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_charging_limit_response() {
        let test = NotifyChargingLimitResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/NotifyChargingLimitResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_customer_information_request() {
        let test = NotifyCustomerInformationRequest {
            custom_data: None,
            data: "".to_string(),
            tbc: Some(false),
            seq_no: 0,
            generated_at: Utc::now(),
            request_id: 0,
        };
        let schema = include_str!("schemas/v2.0.1/NotifyCustomerInformationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_customer_information_response() {
        let test = NotifyCustomerInformationResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/NotifyCustomerInformationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_display_messages_request() {
        let test = NotifyDisplayMessagesRequest {
            custom_data: None,
            request_id: 0,
            tbc: Some(false),
            message_info: Some(vec![MessageInfoType {
                custom_data: None,
                id: 0,
                priority: MessagePriorityEnumType::AlwaysFront,
                state: Some(MessageStateEnumType::Charging),
                start_date_time: Some(Utc::now()),
                end_date_time: Some(Utc::now()),
                transaction_id: Some("transaction_id".to_string()),
                message: MessageContentType {
                    custom_data: None,
                    format: MessageFormatEnumType::ASCII,
                    language: Some("Swedish".to_string()),
                    content: "".to_string(),
                },
                display: Some(ComponentType {
                    custom_data: None,
                    name: "name".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(1),
                    }),
                }),
            }]),
        };
        let schema = include_str!("schemas/v2.0.1/NotifyDisplayMessagesRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_display_messages_response() {
        let test = NotifyDisplayMessagesResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/NotifyDisplayMessagesResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_ev_charging_needs_request() {
        let test = NotifyEVChargingNeedsRequest {
            custom_data: None,
            max_schedule_tuples: Some(0),
            evse_id: 0,
            charging_needs: ChargingNeedsType {
                custom_data: None,
                requested_energy_transfer: EnergyTransferModeEnumType::DC,
                departure_time: Some(Utc::now()),
                ac_charging_parameters: Some(ACChargingParametersType {
                    custom_data: None,
                    energy_amount: 1,
                    ev_min_current: 1,
                    ev_max_current: 1,
                    ev_max_voltage: 1,
                }),
                dc_charging_parameters: Some(DCChargingParametersType {
                    custom_data: None,
                    ev_max_current: 1,
                    ev_max_voltage: 1,
                    energy_amount: Some(0),
                    ev_max_power: Some(0),
                    state_of_charge: Some(100),
                    ev_energy_capacity: Some(0),
                    full_soc: Some(100),
                    bulk_soc: Some(100),
                }),
            },
        };
        let schema = include_str!("schemas/v2.0.1/NotifyEVChargingNeedsRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_ev_charging_needs_response() {
        let test = NotifyEVChargingNeedsResponse {
            custom_data: None,
            status: NotifyEVChargingNeedsStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/NotifyEVChargingNeedsResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_ev_charging_schedule_request() {
        let test = NotifyEVChargingScheduleRequest {
            custom_data: None,
            time_base: Utc::now(),
            evse_id: 0,
            charging_schedule: ChargingScheduleType {
                custom_data: None,
                id: 0,
                start_schedule: Some(Utc::now()),
                duration: Some(0),
                charging_rate_unit: ChargingRateUnitEnumType::W,
                min_charging_rate: Some(dec!(0.0)),
                charging_schedule_period: vec![ChargingSchedulePeriodType {
                    custom_data: None,
                    start_period: 0,
                    limit: dec!(0.0),
                    number_phases: Some(0),
                    phase_to_use: Some(0),
                }],
                sales_tariff: Some(SalesTariffType {
                    custom_data: None,
                    id: 0,
                    sales_tariff_description: Some("sales_tariff_description".to_string()),
                    num_e_price_levels: Some(0),
                    sales_tariff_entry: vec![SalesTariffEntryType {
                        custom_data: None,
                        e_price_level: Some(0),
                        relative_time_interval: RelativeTimeIntervalType {
                            custom_data: None,
                            start: 0,
                            duration: Some(0),
                        },
                        consumption_cost: Some(vec![ConsumptionCostType {
                            custom_data: None,
                            start_value: dec!(0.0),
                            cost: vec![CostType {
                                custom_data: None,
                                cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                amount: 0,
                                amount_multiplier: Some(1),
                            }],
                        }]),
                    }],
                }),
            },
        };
        let schema = include_str!("schemas/v2.0.1/NotifyEVChargingScheduleRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_ev_charging_schedule_response() {
        let test = NotifyEVChargingScheduleResponse {
            custom_data: None,
            status: GenericStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/NotifyEVChargingScheduleResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_event_request() {
        let test = NotifyEventRequest {
            custom_data: None,
            generated_at: Utc::now(),
            tbc: Some(false),
            seq_no: 0,
            event_data: vec![EventDataType {
                custom_data: None,
                event_id: 0,
                timestamp: Utc::now(),
                trigger: EventTriggerEnumType::Alerting,
                cause: Some(0),
                actual_value: "".to_string(),
                tech_code: Some("tech_code".to_string()),
                tech_info: Some("tech_info".to_string()),
                cleared: Some(false),
                transaction_id: Some("transaction_id".to_string()),
                variable_monitoring_id: Some(0),
                event_notification_type: EventNotificationEnumType::HardWiredNotification,
                component: ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("instance".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("instance".to_string()),
                },
            }],
        };
        let schema = include_str!("schemas/v2.0.1/NotifyEventRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_event_response() {
        let test = NotifyEventResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/NotifyEventResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_monitoring_report_request() {
        let test = NotifyMonitoringReportRequest {
            custom_data: None,
            request_id: 0,
            tbc: Some(true),
            seq_no: 0,
            generated_at: Utc::now(),
            monitor: Some(vec![MonitoringDataType {
                custom_data: None,
                component: ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
                variable_monitoring: vec![VariableMonitoringType {
                    custom_data: None,
                    id: 0,
                    transaction: false,
                    value: dec!(0.0),
                    kind: MonitorEnumType::UpperThreshold,
                    severity: 0,
                }],
            }]),
        };
        let schema = include_str!("schemas/v2.0.1/NotifyMonitoringReportRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_monitoring_report_response() {
        let test = NotifyMonitoringReportResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/NotifyMonitoringReportResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    /**
     * Some optional fields including maxLimit of the VariableCharacteristicsType are not
     * included in the payload in order to validate deserialization of optional fields.
     */
    #[test]
    fn validate_notify_report_request_from_json() {
        let json = r#"{
    "generatedAt": "2024-12-23T03:38:31.625Z",
    "reportData": [
      {
        "component": {
          "name": "AlignedDataCtrlr"
        },
        "variable": {
          "name": "Interval"
        },
        "variableAttribute": [
          {
            "mutability": "ReadWrite",
            "value": "10"
          }
        ],
        "variableCharacteristics": {
          "dataType": "integer",
          "supportsMonitoring": true,
          "unit": "seconds"
        }
      },
      {
        "component": {
          "name": "AlignedDataCtrlr"
        },
        "variable": {
          "name": "Measurands"
        },
        "variableAttribute": [
          {
            "mutability": "ReadWrite",
            "value": "Energy.Active.Import.Register"
          }
        ],
        "variableCharacteristics": {
          "dataType": "MemberList",
          "supportsMonitoring": true
        }
      },
      {
        "component": {
          "name": "AlignedDataCtrlr"
        },
        "variable": {
          "name": "TxEndedInterval"
        },
        "variableAttribute": [
          {
            "mutability": "ReadWrite",
            "value": "10"
          }
        ],
        "variableCharacteristics": {
          "dataType": "integer",
          "supportsMonitoring": true,
          "unit": "seconds"
        }
      },
      {
        "component": {
          "name": "AlignedDataCtrlr"
        },
        "variable": {
          "name": "TxEndedMeasurands"
        },
        "variableAttribute": [
          {
            "mutability": "ReadWrite",
            "value": "Energy.Active.Import.Register"
          }
        ],
        "variableCharacteristics": {
          "dataType": "MemberList",
          "supportsMonitoring": true,
          "minLimit": 0
        }
      }
    ],
    "requestId": 1,
    "seqNo": 0,
    "tbc": true
  }"#;

        // verify that the JSON can be deserialized into a NotifyReportRequest object
        let request: NotifyReportRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.request_id, 1);

        let schema = include_str!("schemas/v2.0.1/NotifyReportRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::from_str(json).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_report_request() {
        let test = NotifyReportRequest {
            custom_data: None,
            request_id: 0,
            tbc: Some(false),
            seq_no: 0,
            generated_at: Utc::now(),
            report_data: Some(vec![ReportDataType {
                custom_data: None,
                component: ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "name".to_string(),
                    instance: Some("".to_string()),
                },
                variable_attribute: vec![VariableAttributeType {
                    custom_data: None,
                    kind: Some(AttributeEnumType::Actual),
                    value: Some("value".to_string()),
                    mutability: Some(MutabilityEnumType::ReadOnly),
                    persistent: Some(false),
                    constant: Some(false),
                }],
                variable_characteristics: Some(VariableCharacteristicsType {
                    custom_data: None,
                    unit: Some("unit".to_string()),
                    data_type: DataEnumType::String,
                    min_limit: Some(dec!(0.0)),
                    max_limit: None,
                    values_list: Some("values_list".to_string()),
                    supports_monitoring: false,
                }),
            }]),
        };
        let schema = include_str!("schemas/v2.0.1/NotifyReportRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_notify_report_response() {
        let test = NotifyReportResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/NotifyReportResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_publish_firmware_request() {
        let test = PublishFirmwareRequest {
            custom_data: None,
            location: "".to_string(),
            retries: Some(0),
            checksum: "checksum".to_string(),
            request_id: 0,
            retry_interval: Some(0),
        };
        let schema = include_str!("schemas/v2.0.1/PublishFirmwareRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_publish_firmware_response() {
        let test = PublishFirmwareResponse {
            custom_data: None,
            status: GenericStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/PublishFirmwareResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_publish_firmware_status_notification_request() {
        let test = PublishFirmwareStatusNotificationRequest {
            custom_data: None,
            status: PublishFirmwareStatusEnumType::Idle,
            location: Some(vec!["location".to_string()]),
            request_id: Some(1),
        };
        let schema = include_str!("schemas/v2.0.1/PublishFirmwareStatusNotificationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_publish_firmware_status_notification_response() {
        let test = PublishFirmwareStatusNotificationResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/PublishFirmwareStatusNotificationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_report_charging_profiles_request() {
        let test = ReportChargingProfilesRequest {
            custom_data: None,
            request_id: 0,
            charging_limit_source: ChargingLimitSourceEnumType::EMS,
            tbc: Some(true),
            evse_id: 0,
            charging_profile: vec![ChargingProfileType {
                custom_data: None,
                id: 0,
                stack_level: 0,
                charging_profile_purpose:
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                charging_profile_kind: ChargingProfileKindEnumType::Absolute,
                recurrency_kind: Some(RecurrencyKindEnumType::Daily),
                valid_from: Some(Utc::now()),
                valid_to: Some(Utc::now()),
                transaction_id: Some("transaction_id".to_string()),
                charging_schedule: vec![ChargingScheduleType {
                    custom_data: None,
                    id: 0,
                    start_schedule: Some(Utc::now()),
                    duration: Some(1),
                    charging_rate_unit: ChargingRateUnitEnumType::W,
                    min_charging_rate: Some(dec!(1.0)),
                    charging_schedule_period: vec![ChargingSchedulePeriodType {
                        custom_data: None,
                        start_period: 0,
                        limit: dec!(0.0),
                        number_phases: Some(1),
                        phase_to_use: Some(4),
                    }],
                    sales_tariff: Some(SalesTariffType {
                        custom_data: None,
                        id: 1,
                        sales_tariff_description: Some("sales_tariff_description".to_string()),
                        num_e_price_levels: Some(1),
                        sales_tariff_entry: vec![SalesTariffEntryType {
                            custom_data: None,
                            e_price_level: Some(1),
                            relative_time_interval: RelativeTimeIntervalType {
                                custom_data: None,
                                start: 1,
                                duration: Some(100),
                            },
                            consumption_cost: Some(vec![ConsumptionCostType {
                                custom_data: None,
                                start_value: dec!(0.0),
                                cost: vec![CostType {
                                    custom_data: None,
                                    cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                    amount: 0,
                                    amount_multiplier: Some(1),
                                }],
                            }]),
                        }],
                    }),
                }],
            }],
        };
        let schema = include_str!("schemas/v2.0.1/ReportChargingProfilesRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_report_charging_profiles_response() {
        let test = ReportChargingProfilesResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/ReportChargingProfilesResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_request_start_transaction_request() {
        let test = RequestStartTransactionRequest {
            custom_data: None,
            evse_id: Some(0),
            remote_start_id: 0,
            id_token: IdTokenType {
                custom_data: None,
                id_token: "id_token".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    custom_data: None,
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            },
            charging_profile: Some(ChargingProfileType {
                custom_data: None,
                id: 0,
                stack_level: 0,
                charging_profile_purpose:
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                charging_profile_kind: ChargingProfileKindEnumType::Absolute,
                recurrency_kind: Some(RecurrencyKindEnumType::Daily),
                valid_from: Some(Utc::now()),
                valid_to: Some(Utc::now()),
                transaction_id: Some("transaction_id".to_string()),
                charging_schedule: vec![ChargingScheduleType {
                    custom_data: None,
                    id: 0,
                    start_schedule: Some(Utc::now()),
                    duration: Some(1),
                    charging_rate_unit: ChargingRateUnitEnumType::W,
                    min_charging_rate: Some(dec!(0.1)),
                    charging_schedule_period: vec![ChargingSchedulePeriodType {
                        custom_data: None,
                        start_period: 0,
                        limit: dec!(0.0),
                        number_phases: Some(1),
                        phase_to_use: Some(1),
                    }],
                    sales_tariff: Some(SalesTariffType {
                        custom_data: None,
                        id: 1,
                        sales_tariff_description: Some("".to_string()),
                        num_e_price_levels: Some(2),
                        sales_tariff_entry: vec![SalesTariffEntryType {
                            custom_data: None,
                            e_price_level: Some(1),
                            relative_time_interval: RelativeTimeIntervalType {
                                custom_data: None,
                                start: 0,
                                duration: Some(0),
                            },
                            consumption_cost: Some(vec![ConsumptionCostType {
                                custom_data: None,
                                start_value: dec!(0.0),
                                cost: vec![CostType {
                                    custom_data: None,
                                    cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                    amount: 0,
                                    amount_multiplier: Some(1),
                                }],
                            }]),
                        }],
                    }),
                }],
            }),
            group_id_token: Some(IdTokenType {
                custom_data: None,
                id_token: "id_token".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    custom_data: None,
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/RequestStartTransactionRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_request_start_transaction_response() {
        let test = RequestStartTransactionResponse {
            custom_data: None,
            status: RequestStartStopStatusEnumType::Accepted,
            transaction_id: Some("".to_string()),
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/RequestStartTransactionResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_request_stop_transaction_request() {
        let test = RequestStopTransactionRequest {
            custom_data: None,
            transaction_id: "".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/RequestStopTransactionRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_request_stop_transaction_response() {
        let test = RequestStopTransactionResponse {
            custom_data: None,
            status: RequestStartStopStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/RequestStopTransactionResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reservation_status_update_request() {
        let test = ReservationStatusUpdateRequest {
            custom_data: None,
            reservation_id: 0,
            reservation_update_status: ReservationUpdateStatusEnumType::Expired,
        };
        let schema = include_str!("schemas/v2.0.1/ReservationStatusUpdateRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reservation_status_update_response() {
        let test = ReservationStatusUpdateResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/ReservationStatusUpdateResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reserve_now_request() {
        let test = ReserveNowRequest {
            custom_data: None,
            id: 0,
            expiry_date_time: Utc::now(),
            connector_type: Some(ConnectorEnumType::CCCS1),
            evse_id: Some(0),
            id_token: IdTokenType {
                custom_data: None,
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    custom_data: None,
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            },
            group_id_token: Some(IdTokenType {
                custom_data: None,
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    custom_data: None,
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/ReserveNowRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reserve_now_response() {
        let test = ReserveNowResponse {
            custom_data: None,
            status: ReserveNowStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/ReserveNowResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reset_request() {
        let test = ResetRequest {
            custom_data: None,
            kind: ResetEnumType::Immediate,
            evse_id: Some(0),
        };
        let schema = include_str!("schemas/v2.0.1/ResetRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_reset_response() {
        let test = ResetResponse {
            custom_data: None,
            status: ResetStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/ResetResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_security_event_notification_request() {
        let test = SecurityEventNotificationRequest {
            custom_data: None,
            kind: "".to_string(),
            timestamp: Utc::now(),
            tech_info: Some("".to_string()),
        };
        let schema = include_str!("schemas/v2.0.1/SecurityEventNotificationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_security_event_notification_response() {
        let test = SecurityEventNotificationResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/SecurityEventNotificationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_send_local_list_request() {
        let test = SendLocalListRequest {
            custom_data: None,
            version_number: 0,
            update_type: UpdateEnumType::Differential,
            local_authorization_list: Some(vec![AuthorizationData {
                custom_data: None,
                id_token_info: Some(IdTokenInfoType {
                    custom_data: None,
                    status: AuthorizationStatusEnumType::Accepted,
                    cache_expiry_date_time: Some(Utc::now()),
                    charging_priority: Some(0),
                    language1: Some("lang1".to_string()),
                    evse_id: Some(vec![1, 2, 3]),
                    language2: Some("lang2".to_string()),
                    group_id_token: Some(IdTokenType {
                        custom_data: None,
                        id_token: "id_token".to_string(),
                        kind: IdTokenEnumType::Central,
                        additional_info: Some(vec![AdditionalInfoType {
                            custom_data: None,
                            additional_id_token: "additional_id_token".to_string(),
                            kind: "type".to_string(),
                        }]),
                    }),
                    personal_message: Some(MessageContentType {
                        custom_data: None,
                        format: MessageFormatEnumType::ASCII,
                        language: Some("English".to_string()),
                        content: "Hello, world!".to_string(),
                    }),
                }),
                id_token: IdTokenType {
                    custom_data: None,
                    id_token: "".to_string(),
                    kind: IdTokenEnumType::Central,
                    additional_info: Some(vec![AdditionalInfoType {
                        custom_data: None,
                        additional_id_token: "additional_id_token".to_string(),
                        kind: "type".to_string(),
                    }]),
                },
            }]),
        };
        let schema = include_str!("schemas/v2.0.1/SendLocalListRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_send_local_list_response() {
        let test = SendLocalListResponse {
            custom_data: None,
            status: SendLocalListStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/SendLocalListResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_charging_profile_request() {
        let test = SetChargingProfileRequest {
            custom_data: None,
            evse_id: 0,
            charging_profile: ChargingProfileType {
                custom_data: None,
                id: 0,
                stack_level: 0,
                charging_profile_purpose:
                    ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
                charging_profile_kind: ChargingProfileKindEnumType::Absolute,
                recurrency_kind: Some(RecurrencyKindEnumType::Daily),
                valid_from: Some(Utc::now()),
                valid_to: Some(Utc::now()),
                transaction_id: Some("".to_string()),
                charging_schedule: vec![ChargingScheduleType {
                    custom_data: None,
                    id: 0,
                    start_schedule: Some(Utc::now()),
                    duration: Some(0),
                    charging_rate_unit: ChargingRateUnitEnumType::W,
                    min_charging_rate: Some(dec!(0.0)),
                    charging_schedule_period: vec![ChargingSchedulePeriodType {
                        custom_data: None,
                        start_period: 0,
                        limit: dec!(0.0),
                        number_phases: Some(0),
                        phase_to_use: Some(0),
                    }],
                    sales_tariff: Some(SalesTariffType {
                        custom_data: None,
                        id: 0,
                        sales_tariff_description: Some("".to_string()),
                        num_e_price_levels: Some(0),
                        sales_tariff_entry: vec![SalesTariffEntryType {
                            custom_data: None,
                            e_price_level: Some(0),
                            relative_time_interval: RelativeTimeIntervalType {
                                custom_data: None,
                                start: 0,
                                duration: Some(0),
                            },
                            consumption_cost: Some(vec![ConsumptionCostType {
                                custom_data: None,
                                start_value: dec!(0.0),
                                cost: vec![CostType {
                                    custom_data: None,
                                    cost_kind: CostKindEnumType::CarbonDioxideEmission,
                                    amount: 0,
                                    amount_multiplier: Some(0),
                                }],
                            }]),
                        }],
                    }),
                }],
            },
        };
        let schema = include_str!("schemas/v2.0.1/SetChargingProfileRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_charging_profile_response() {
        let test = SetChargingProfileResponse {
            custom_data: None,
            status: ChargingProfileStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/SetChargingProfileResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_display_message_request() {
        let test = SetDisplayMessageRequest {
            custom_data: None,
            message: MessageInfoType {
                custom_data: None,
                id: 0,
                priority: MessagePriorityEnumType::AlwaysFront,
                state: Some(MessageStateEnumType::Charging),
                start_date_time: Some(Utc::now()),
                end_date_time: Some(Utc::now()),
                transaction_id: Some("".to_string()),
                message: MessageContentType {
                    custom_data: None,
                    format: MessageFormatEnumType::ASCII,
                    language: Some("lang".to_string()),
                    content: "".to_string(),
                },
                display: Some(ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                }),
            },
        };
        let schema = include_str!("schemas/v2.0.1/SetDisplayMessageRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_display_message_response() {
        let test = SetDisplayMessageResponse {
            custom_data: None,
            status: DisplayMessageStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/SetDisplayMessageResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_monitoring_base_request() {
        let test = SetMonitoringBaseRequest {
            custom_data: None,
            monitoring_base: MonitoringBaseEnumType::All,
        };
        let schema = include_str!("schemas/v2.0.1/SetMonitoringBaseRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_monitoring_base_response() {
        let test = SetMonitoringBaseResponse {
            custom_data: None,
            status: GenericDeviceModelStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/SetMonitoringBaseResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_monitoring_level_request() {
        let test = SetMonitoringLevelRequest {
            custom_data: None,
            severity: 0,
        };
        let schema = include_str!("schemas/v2.0.1/SetMonitoringLevelRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_monitoring_level_response() {
        let test = SetMonitoringLevelResponse {
            custom_data: None,
            status: GenericStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/SetMonitoringLevelResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_network_profile_request() {
        let test = SetNetworkProfileRequest {
            custom_data: None,
            configuration_slot: 0,
            connection_data: NetworkConnectionProfileType {
                custom_data: None,
                ocpp_version: OCPPVersionEnumType::OCPP12,
                ocpp_transport: OCPPTransportEnumType::JSON,
                ocpp_csms_url: "".to_string(),
                message_timeout: 0,
                security_profile: 0,
                ocpp_interface: OCPPInterfaceEnumType::Wired0,
                vpn: Some(VPNType {
                    custom_data: None,
                    server: "server".to_string(),
                    user: "user".to_string(),
                    group: Some("group".to_string()),
                    password: "password".to_string(),
                    key: "key".to_string(),
                    kind: VPNEnumType::IKEv2,
                }),
                apn: Some(APNType {
                    custom_data: None,
                    apn: "apn".to_string(),
                    apn_user_name: Some("apn_user_name".to_string()),
                    apn_password: Some("apn_password".to_string()),
                    sim_pin: Some(1),
                    preferred_network: Some("6chars".to_string()),
                    use_only_preferred_network: Some(false),
                    apn_authentication: APNAuthenticationEnumType::CHAP,
                }),
            },
        };
        let schema = include_str!("schemas/v2.0.1/SetNetworkProfileRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_network_profile_response() {
        let test = SetNetworkProfileResponse {
            custom_data: None,
            status: SetNetworkProfileStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/SetNetworkProfileResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_variable_monitoring_request() {
        let test = SetVariableMonitoringRequest {
            custom_data: None,
            set_monitoring_data: vec![SetMonitoringDataType {
                custom_data: None,
                id: Some(0),
                transaction: Some(false),
                value: dec!(0.0),
                kind: MonitorEnumType::UpperThreshold,
                severity: 0,
                component: ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
            }],
        };
        let schema = include_str!("schemas/v2.0.1/SetVariableMonitoringRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_variable_monitoring_response() {
        let test = SetVariableMonitoringResponse {
            custom_data: None,
            set_monitoring_result: vec![SetMonitoringResultType {
                custom_data: None,
                id: Some(0),
                status: SetMonitoringStatusEnumType::Accepted,
                kind: MonitorEnumType::UpperThreshold,
                severity: 0,
                component: ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
                status_info: Some(StatusInfoType {
                    custom_data: None,
                    reason_code: "".to_string(),
                    additional_info: Some("".to_string()),
                }),
            }],
        };
        let schema = include_str!("schemas/v2.0.1/SetVariableMonitoringResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_variables_request() {
        let test = SetVariablesRequest {
            custom_data: None,
            set_variable_data: vec![SetVariableDataType {
                custom_data: None,
                attribute_type: Some(AttributeEnumType::Actual),
                attribute_value: "".to_string(),
                component: ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
            }],
        };
        let schema = include_str!("schemas/v2.0.1/SetVariablesRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_set_variables_response() {
        let test = SetVariablesResponse {
            custom_data: None,
            set_variable_result: vec![SetVariableResultType {
                custom_data: None,
                attribute_type: Some(AttributeEnumType::Actual),
                attribute_status: SetVariableStatusEnumType::Accepted,
                component: ComponentType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                    evse: Some(EVSEType {
                        custom_data: None,
                        id: 0,
                        connector_id: Some(0),
                    }),
                },
                variable: VariableType {
                    custom_data: None,
                    name: "".to_string(),
                    instance: Some("".to_string()),
                },
                attribute_status_info: Some(StatusInfoType {
                    custom_data: None,
                    reason_code: "".to_string(),
                    additional_info: Some("".to_string()),
                }),
            }],
        };
        let schema = include_str!("schemas/v2.0.1/SetVariablesResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_sign_certificate_request() {
        let test = SignCertificateRequest {
            custom_data: None,
            csr: "".to_string(),
            certificate_type: Some(CertificateSigningUseEnumType::ChargingStationCertificate),
        };
        let schema = include_str!("schemas/v2.0.1/SignCertificateRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_sign_certificate_response() {
        let test = SignCertificateResponse {
            custom_data: None,
            status: GenericStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/SignCertificateResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_status_notification_request() {
        let test = StatusNotificationRequest {
            custom_data: None,
            timestamp: Utc::now(),
            connector_status: ConnectorStatusEnumType::Available,
            evse_id: 0,
            connector_id: 0,
        };
        let schema = include_str!("schemas/v2.0.1/StatusNotificationRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_status_notification_response() {
        let test = StatusNotificationResponse { custom_data: None };
        let schema = include_str!("schemas/v2.0.1/StatusNotificationResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_transaction_event_request() {
        let test = TransactionEventRequest {
            custom_data: None,
            event_type: TransactionEventEnumType::Ended,
            timestamp: Utc::now(),
            trigger_reason: TriggerReasonEnumType::Authorized,
            seq_no: 0,
            offline: Some(false),
            number_of_phases_used: Some(0),
            cable_max_current: Some(0),
            reservation_id: Some(0),
            transaction_info: TransactionType {
                custom_data: None,
                transaction_id: "".to_string(),
                charging_state: Some(ChargingStateEnumType::Charging),
                time_spent_charging: Some(0),
                stopped_reason: Some(ReasonEnumType::DeAuthorized),
                remote_start_id: Some(0),
            },
            id_token: Some(IdTokenType {
                custom_data: None,
                id_token: "".to_string(),
                kind: IdTokenEnumType::Central,
                additional_info: Some(vec![AdditionalInfoType {
                    custom_data: None,
                    additional_id_token: "".to_string(),
                    kind: "".to_string(),
                }]),
            }),
            evse: Some(EVSEType {
                custom_data: None,
                id: 0,
                connector_id: Some(0),
            }),
            meter_value: Some(vec![MeterValueType {
                custom_data: None,
                timestamp: Utc::now(),
                sampled_value: vec![SampledValueType {
                    custom_data: None,
                    value: dec!(0.0),
                    context: Some(ReadingContextEnumType::InterruptionBegin),
                    measurand: Some(MeasurandEnumType::CurrentExport),
                    phase: Some(PhaseEnumType::L1),
                    location: Some(LocationEnumType::Body),
                    signed_meter_value: Some(SignedMeterValueType {
                        custom_data: None,
                        signed_meter_data: "signed_meter_data".to_string(),
                        signing_method: "signing_method".to_string(),
                        encoding_method: "encoding_method".to_string(),
                        public_key: "public_key".to_string(),
                    }),
                    unit_of_measure: Some(UnitOfMeasureType {
                        custom_data: None,
                        unit: Some("unit".to_string()),
                        multiplier: Some(0),
                    }),
                }],
            }]),
        };
        let schema = include_str!("schemas/v2.0.1/TransactionEventRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_transaction_event_response() {
        let test = TransactionEventResponse {
            custom_data: None,
            total_cost: Some(dec!(0.0)),
            charging_priority: Some(0),
            id_token_info: Some(IdTokenInfoType {
                custom_data: None,
                status: AuthorizationStatusEnumType::Accepted,
                cache_expiry_date_time: Some(Utc::now()),
                charging_priority: Some(0),
                language1: Some("".to_string()),
                evse_id: Some(vec![1]),
                language2: Some("".to_string()),
                group_id_token: Some(IdTokenType {
                    custom_data: None,
                    id_token: "".to_string(),
                    kind: IdTokenEnumType::Central,
                    additional_info: Some(vec![AdditionalInfoType {
                        custom_data: None,
                        additional_id_token: "additional_id_token".to_string(),
                        kind: "type".to_string(),
                    }]),
                }),
                personal_message: Some(MessageContentType {
                    custom_data: None,
                    format: MessageFormatEnumType::ASCII,
                    language: Some("language".to_string()),
                    content: "content".to_string(),
                }),
            }),
            updated_personal_message: Some(MessageContentType {
                custom_data: None,
                format: MessageFormatEnumType::ASCII,
                language: Some("language".to_string()),
                content: "content".to_string(),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/TransactionEventResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_trigger_message_request() {
        let test = TriggerMessageRequest {
            custom_data: None,
            requested_message: MessageTriggerEnumType::BootNotification,
            evse: Some(EVSEType {
                custom_data: None,
                id: 0,
                connector_id: Some(0),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/TriggerMessageRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_trigger_message_response() {
        let test = TriggerMessageResponse {
            custom_data: None,
            status: TriggerMessageStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/TriggerMessageResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_unlock_connector_request() {
        let test = UnlockConnectorRequest {
            custom_data: None,
            evse_id: 0,
            connector_id: 0,
        };
        let schema = include_str!("schemas/v2.0.1/UnlockConnectorRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_unlock_connector_response() {
        let test = UnlockConnectorResponse {
            custom_data: None,
            status: UnlockStatusEnumType::Unlocked,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/UnlockConnectorResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_unpublish_firmware_request() {
        let test = UnpublishFirmwareRequest {
            custom_data: None,
            checksum: "".to_string(),
        };
        let schema = include_str!("schemas/v2.0.1/UnpublishFirmwareRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_unpublish_firmware_response() {
        let test = UnpublishFirmwareResponse {
            custom_data: None,
            status: UnpublishFirmwareStatusEnumType::DownloadOngoing,
        };
        let schema = include_str!("schemas/v2.0.1/UnpublishFirmwareResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_update_firmware_request() {
        let test = UpdateFirmwareRequest {
            custom_data: None,
            retries: Some(0),
            retry_interval: Some(0),
            request_id: 0,
            firmware: FirmwareType {
                custom_data: None,
                location: "".to_string(),
                retrieve_date_time: Utc::now(),
                install_date_time: Some(Utc::now()),
                signing_certificate: Some("signing_certificate".to_string()),
                signature: Some("signature".to_string()),
            },
        };
        let schema = include_str!("schemas/v2.0.1/UpdateFirmwareRequest.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
    #[test]
    fn validate_update_firmware_response() {
        let test = UpdateFirmwareResponse {
            custom_data: None,
            status: UpdateFirmwareStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                custom_data: None,
                reason_code: "".to_string(),
                additional_info: Some("".to_string()),
            }),
        };
        let schema = include_str!("schemas/v2.0.1/UpdateFirmwareResponse.json");
        let schema = serde_json::from_str(schema).unwrap();
        let instance = serde_json::to_value(test).unwrap();
        let compiled = Validator::new(&schema).expect("A valid schema");
        let result = compiled.validate(&instance);
        if result.is_err() {
            for error in compiled.iter_errors(&instance) {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path());
            }
        }
        assert!(compiled.is_valid(&instance));
    }
}
