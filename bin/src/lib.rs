use chrono::NaiveTime;
use core_payment_plan::{Params, Response};
use prost::Message;
use types::{PlanParams, PlanResponse, PlanResponses};

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/cli.types.rs"));
}

impl TryInto<Params> for PlanParams {
    type Error = String;

    fn try_into(self) -> Result<Params, Self::Error> {
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

        let params = Params {
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

impl From<Response> for PlanResponse {
    fn from(value: Response) -> Self {
        let due_date = value
            .due_date
            .and_time(NaiveTime::from_hms_opt(3, 0, 0).unwrap())
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

impl From<Vec<Response>> for PlanResponses {
    fn from(value: Vec<Response>) -> Self {
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
