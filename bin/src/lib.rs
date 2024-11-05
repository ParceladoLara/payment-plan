use chrono::NaiveTime;
use core_payment_plan::types::{down_payment_plan, plan, reimbursement};
use prost::Message;
use types::{
    DownPaymentParams, DownPaymentResponse, DownPaymentResponses, InvoiceParamReimbursement,
    InvoiceResponseReimbursement, InvoiceStatusReimbursement, PlanParams, PlanResponse,
    PlanResponses, ReimbursementParams, ReimbursementResponse,
};

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/cli.types.rs"));
}

impl TryInto<plan::Params> for PlanParams {
    type Error = String;

    fn try_into(self) -> Result<plan::Params, Self::Error> {
        let first_payment_date =
            chrono::DateTime::from_timestamp_millis(self.first_payment_date_millis);
        let first_payment_date = match first_payment_date {
            Some(date) => date.date_naive(),
            None => {
                return Err("invalid first payment date".to_string());
            }
        };

        let requested_date = chrono::DateTime::from_timestamp_millis(self.requested_date_millis);
        let requested_date = match requested_date {
            Some(date) => date.date_naive(),
            None => {
                return Err("invalid requested date".to_string());
            }
        };

        let params = plan::Params {
            max_total_amount: self.max_total_amount,
            min_installment_amount: self.min_installment_amount,
            requested_amount: self.requested_amount,
            debit_service_percentage: self.debit_service_percentage as u16,
            installments: self.installments,
            interest_rate: self.interest_rate,
            iof_overall: self.iof_overall,
            iof_percentage: self.iof_percentage,
            mdr: self.mdr,
            tac_percentage: self.tac_percentage,
            first_payment_date,
            requested_date,
        };
        return Ok(params);
    }
}

impl From<plan::Response> for PlanResponse {
    fn from(value: plan::Response) -> Self {
        let due_date = value
            .due_date
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp_millis();

        PlanResponse {
            accumulated_days: value.accumulated_days,
            days_index: value.days_index,
            due_date_millis: due_date,
            accumulated_days_index: value.accumulated_days_index,
            calculation_basis_for_effective_interest_rate: value
                .calculation_basis_for_effective_interest_rate,
            contract_amount: value.contract_amount,
            customer_amount: value.customer_amount,
            customer_debit_service_amount: value.customer_debit_service_amount,
            debit_service: value.debit_service,
            installment: value.installment,
            installment_amount: value.installment_amount,
            installment_amount_without_tac: value.installment_amount_without_tac,
            interest_rate: value.interest_rate,
            total_amount: value.total_amount,
            contract_amount_without_tac: value.contract_amount_without_tac,
            effective_interest_rate: value.effective_interest_rate,
            total_iof: value.total_iof,
            total_effective_cost: value.total_effective_cost,
            tec_yearly: value.tec_yearly,
            tec_monthly: value.tec_monthly,
            eir_yearly: value.eir_yearly,
            eir_monthly: value.eir_monthly,
            settled_to_merchant: value.settled_to_merchant,
            merchant_total_amount: value.merchant_total_amount,
            merchant_debit_service_amount: value.merchant_debit_service_amount,
            tac_amount: value.tac_amount,
            mdr_amount: value.mdr_amount,
            overall_iof: value.overall_iof,
            iof_percentage: value.iof_percentage,
        }
    }
}

impl From<Vec<plan::Response>> for PlanResponses {
    fn from(value: Vec<plan::Response>) -> Self {
        let responses = value.into_iter().map(|r| r.into()).collect();
        PlanResponses { responses }
    }
}

pub fn deserialize_params(buf: &[u8]) -> Result<PlanParams, prost::DecodeError> {
    PlanParams::decode(buf)
}

pub fn serialize_response(response: PlanResponse) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(response.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    response.encode(&mut buf).unwrap();
    buf
}

pub fn serialize_responses(responses: PlanResponses) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(responses.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    responses.encode(&mut buf).unwrap();
    buf
}

impl TryInto<down_payment_plan::Params> for DownPaymentParams {
    type Error = String;

    fn try_into(self) -> Result<down_payment_plan::Params, Self::Error> {
        let first_payment_date =
            chrono::DateTime::from_timestamp_millis(self.first_payment_date_millis);
        let first_payment_date = match first_payment_date {
            Some(date) => date.date_naive(),
            None => {
                return Err("invalid first payment date".to_string());
            }
        };

        let params = self.params.ok_or("missing params")?;
        let params: plan::Params = params.try_into()?;
        let down_payment_params = down_payment_plan::Params {
            params,
            first_payment_date,
            installments: self.installments,
            min_installment_amount: self.min_installment_amount,
            requested_amount: self.requested_amount,
        };

        Ok(down_payment_params)
    }
}

impl From<down_payment_plan::Response> for DownPaymentResponse {
    fn from(value: down_payment_plan::Response) -> Self {
        let plan: PlanResponses = value.plans.into();

        let first_payment_date = value
            .first_payment_date
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp_millis();

        DownPaymentResponse {
            first_payment_date_millis: first_payment_date,
            plans: Some(plan),
            installment_amount: value.installment_amount,
            total_amount: value.total_amount,
            installment_quantity: value.installment_quantity,
        }
    }
}

impl From<Vec<down_payment_plan::Response>> for DownPaymentResponses {
    fn from(value: Vec<down_payment_plan::Response>) -> Self {
        let responses = value.into_iter().map(|r| r.into()).collect();
        DownPaymentResponses { responses }
    }
}

pub fn deserialize_down_payment_params(
    buf: &[u8],
) -> Result<DownPaymentParams, prost::DecodeError> {
    DownPaymentParams::decode(buf)
}

pub fn serialize_down_payment_response(response: DownPaymentResponse) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(response.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    response.encode(&mut buf).unwrap();
    buf
}

pub fn serialize_down_payment_responses(responses: DownPaymentResponses) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(responses.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    responses.encode(&mut buf).unwrap();
    buf
}

impl Into<reimbursement::InvoiceStatus> for InvoiceStatusReimbursement {
    fn into(self) -> reimbursement::InvoiceStatus {
        match self {
            InvoiceStatusReimbursement::Created => reimbursement::InvoiceStatus::CREATED,
            InvoiceStatusReimbursement::Overdue => reimbursement::InvoiceStatus::OVERDUE,
            InvoiceStatusReimbursement::Paid => reimbursement::InvoiceStatus::PAID,
            InvoiceStatusReimbursement::Readjusted => reimbursement::InvoiceStatus::READJUSTED,
        }
    }
}

impl TryInto<reimbursement::InvoiceParam> for InvoiceParamReimbursement {
    type Error = String;

    fn try_into(self) -> Result<reimbursement::InvoiceParam, Self::Error> {
        let due_at = chrono::DateTime::from_timestamp_millis(self.due_at_millis);
        let due_at = match due_at {
            Some(date) => date.date_naive(),
            None => {
                return Err("invalid due at date".to_string());
            }
        };

        let status: reimbursement::InvoiceStatus =
            types::InvoiceStatusReimbursement::try_from(self.status)
                .map_err(|_| format!("invalid invoice status: {}", self.status))?
                .into();

        let invoice = reimbursement::InvoiceParam {
            id: self.id,
            status,
            original_amount: self.original_amount,
            due_at,
            main_iof_tac: self.main_iof_tac,
        };

        Ok(invoice)
    }
}

impl TryInto<reimbursement::Params> for ReimbursementParams {
    type Error = String;

    fn try_into(self) -> Result<reimbursement::Params, Self::Error> {
        let invoices = self
            .invoices
            .into_iter()
            .map(|invoice| invoice.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        let base_date = chrono::DateTime::from_timestamp_millis(self.base_date_millis);
        let base_date = match base_date {
            Some(date) => date.date_naive(),
            None => {
                return Err("invalid base date".to_string());
            }
        };

        let params = reimbursement::Params {
            base_date,
            interest_rate: self.interest_rate,
            max_repurchase_payment_days: self.max_repurchase_payment_days,
            max_reimbursement_payment_days: self.max_reimbursement_payment_days,
            invoices,
            fee: self.fee,
            invoice_cost: self.invoice_cost,
            mdr: self.mdr,
        };

        Ok(params)
    }
}

impl From<reimbursement::InvoiceResponse> for InvoiceResponseReimbursement {
    fn from(value: reimbursement::InvoiceResponse) -> Self {
        return InvoiceResponseReimbursement {
            days_difference_between_repurchase_date_and_due_at: value
                .days_difference_between_repurchase_date_and_due_at,
            id: value.id,
            present_value_repurchase: value.present_value_repurchase,
        };
    }
}

impl From<reimbursement::Response> for ReimbursementResponse {
    fn from(value: reimbursement::Response) -> Self {
        let invoices = value
            .invoices
            .into_iter()
            .map(|invoice| invoice.into())
            .collect();

        let reimbursement_invoice_due_date_millis = value
            .reimbursement_invoice_due_date
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp_millis();

        let reference_date_for_repurchase_millis = value
            .reference_date_for_repurchase
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp_millis();

        ReimbursementResponse {
            customer_charge_back_amount: value.customer_charge_back_amount,
            total_present_value_repurchase: value.total_present_value_repurchase,
            subsidy_for_cancellation: value.subsidy_for_cancellation,
            reimbursement_value: value.reimbursement_value,
            reimbursement_invoice_due_date_millis,
            reference_date_for_repurchase_millis,
            invoices,
            interest_rate_daily: value.interest_rate_daily,
        }
    }
}

pub fn deserialize_reimbursement_params(
    buf: &[u8],
) -> Result<ReimbursementParams, prost::DecodeError> {
    ReimbursementParams::decode(buf)
}

pub fn serialize_reimbursement_response(response: ReimbursementResponse) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(response.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    response.encode(&mut buf).unwrap();
    buf
}
