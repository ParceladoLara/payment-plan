package payment_plan

import "github.com/ParceladoLara/payment-plan-go-sdk/internal/payment_plan_uniffi"

type Params payment_plan_uniffi.Params
type Response payment_plan_uniffi.Response
type DownPaymentParams payment_plan_uniffi.DownPaymentParams
type DownPaymentResponse payment_plan_uniffi.DownPaymentResponse

func CalculatePaymentPlan(params Params) ([]Response, error) {
	response, err := payment_plan_uniffi.CalculatePaymentPlan(payment_plan_uniffi.Params(params))
	if err != nil {
		return nil, err
	}
	convertedResponse := make([]Response, len(response))
	for i, r := range response {
		convertedResponse[i] = Response(r)
	}
	return convertedResponse, nil
}

func CalculateDownPayment(params DownPaymentParams) ([]DownPaymentResponse, error) {
	response, err := payment_plan_uniffi.CalculateDownPaymentPlan(payment_plan_uniffi.DownPaymentParams(params))
	if err != nil {
		return nil, err
	}
	convertedResponse := make([]DownPaymentResponse, len(response))
	for i, r := range response {
		convertedResponse[i] = DownPaymentResponse(r)
	}
	return convertedResponse, nil
}
