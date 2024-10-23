package bin_test

import (
	"bin/protos"
	"bytes"
	"errors"
	"fmt"
	"os/exec"
	"testing"
	"time"

	"google.golang.org/protobuf/proto"
)

var firstPaymentDate = time.Date(2022, time.April, 18, 0, 0, 0, 0, time.Local)
var requestedDate = time.Date(2022, time.March, 18, 0, 0, 0, 0, time.Local)

var param = protos.PlanParams{
	RequestedAmount:        8800,
	FirstPaymentDateMillis: firstPaymentDate.UnixMilli(),
	RequestedDateMillis:    requestedDate.UnixMilli(),
	Installments:           24,
	DebitServicePercentage: 0,
	Mdr:                    0.05,
	TacPercentage:          0,
	IofOverall:             0.0038,
	IofPercentage:          0.03,
	InterestRate:           0.0235,
	MinInstallmentAmount:   100,
	MaxTotalAmount:         1000000,
}

var qiTechParam = protos.PlanParams{
	RequestedAmount:        8800,
	FirstPaymentDateMillis: firstPaymentDate.UnixMilli(),
	RequestedDateMillis:    requestedDate.UnixMilli(),
	Installments:           24,
	DebitServicePercentage: 0,
	Mdr:                    0.05,
	TacPercentage:          0,
	IofOverall:             0.0038,
	IofPercentage:          0.000082,
	InterestRate:           0.0235,
	MinInstallmentAmount:   100,
	MaxTotalAmount:         1000000,
}
var downPaymentParams = protos.DownPaymentParams{
	Params:                 &param,
	FirstPaymentDateMillis: time.Date(2022, time.June, 20, 0, 0, 0, 0, time.UTC).UnixMilli(),
	RequestedAmount:        200,
	Installments:           2,
	MinInstallmentAmount:   100,
}

var qiTechDownPaymentParams = protos.DownPaymentParams{
	Params:                 &qiTechParam,
	FirstPaymentDateMillis: time.Date(2022, time.June, 20, 0, 0, 0, 0, time.UTC).UnixMilli(),
	RequestedAmount:        200,
	Installments:           2,
	MinInstallmentAmount:   100,
}

type expectedValues struct {
	ContractAmount             float64
	ContractAmountWithoutTac   float64
	CustomerAmount             float64
	CustomerDebitServiceAmount float64
	DebitService               float64
	EirMonthly                 float64
	EirYearly                  float64
	EffectiveInterestRate      float64
	Installment                int
	InstallmentAmount          float64
	InterestRate               float64
	MdrAmount                  float64
	MerchantDebitServiceAmount float64
	MerchantTotalAmount        float64
	SettledToMerchant          float64
	TecMonthly                 float64
	TecYearly                  float64
	TotalAmount                float64
	TotalIof                   float64
}

func TestBMPPlan(t *testing.T) {

	expected := expectedValues{
		ContractAmount:             9037.318869753424,
		ContractAmountWithoutTac:   9037.318869753424,
		CustomerAmount:             499.1987614851067,
		CustomerDebitServiceAmount: 2943.451405889136,
		DebitService:               2943.451405889136,
		EirMonthly:                 0.024085088183680048,
		EirYearly:                  0.33055401101326365,
		EffectiveInterestRate:      0.024085088183680048,
		Installment:                24,
		InstallmentAmount:          499.1987614851067,
		InterestRate:               0.0235,
		MdrAmount:                  440.0,
		MerchantDebitServiceAmount: 0.0,
		MerchantTotalAmount:        440.0,
		SettledToMerchant:          8360.0,
		TecMonthly:                 0.025868426671143974,
		TecYearly:                  0.3586261331729559,
		TotalAmount:                11980.77027564256,
		TotalIof:                   237.3188697534247,
	}

	var plan protos.PlanResponses

	err := callBuff(&param, &plan)
	if err != nil {
		t.Errorf("Error running payment-plan CLI: %s", err)
	}

	t.Run("Success", func(t *testing.T) {

		planHelper(t, &plan, &expected)
	})

	t.Run("invalid installments", func(t *testing.T) {
		oldInstallments := param.Installments
		param.Installments = 0
		err := callBuff(&param, &plan)
		param.Installments = oldInstallments
		if err == nil {
			t.Errorf("Expected error, got none")
		}
	})

	t.Run("invalid requested amount", func(t *testing.T) {
		oldRequestedAmount := param.RequestedAmount
		param.RequestedAmount = 0
		err := callBuff(&param, &plan)
		param.RequestedAmount = oldRequestedAmount
		if err == nil {
			t.Errorf("Expected error, got none")
		}
	})

}
func TestQiTechPlan(t *testing.T) {
	var plan protos.PlanResponses

	err := callBuff(&qiTechParam, &plan)
	if err != nil {
		t.Errorf("Error running payment-plan CLI: %s", err)
	}
}

func TestBMPDownPayment(t *testing.T) {
	var plan protos.DownPaymentResponses

	err := callBuff(&downPaymentParams, &plan, "-t", "down-payment")
	if err != nil {
		t.Fatalf("Error running payment-plan CLI: %s", err)
	}

	downPaymentHelper(t, &plan)

}

func TestQiTechDownPayment(t *testing.T) {
	var plan protos.DownPaymentResponses

	err := callBuff(&qiTechDownPaymentParams, &plan, "-t", "down-payment")
	if err != nil {
		t.Fatalf("Error running payment-plan CLI: %s", err)
	}

	downPaymentHelper(t, &plan)
}

func planHelper(t *testing.T, plan *protos.PlanResponses, expected *expectedValues) {

	if len(plan.Responses) != expected.Installment {
		t.Fatalf("Expected %d response, got %d", expected.Installment, len(plan.Responses))
	}

	response := plan.Responses[expected.Installment-1]

	if response.ContractAmount != expected.ContractAmount {
		t.Errorf("Expected contract amount to be %f, got %f", expected.ContractAmount, response.ContractAmount)
	}

	if response.ContractAmountWithoutTac != expected.ContractAmountWithoutTac {
		t.Errorf("Expected contract amount without tac to be %f, got %f", expected.ContractAmountWithoutTac, response.ContractAmountWithoutTac)
	}

	if response.CustomerAmount != expected.CustomerAmount {
		t.Errorf("Expected customer amount to be %f, got %f", expected.CustomerAmount, response.CustomerAmount)
	}

	if response.CustomerDebitServiceAmount != expected.CustomerDebitServiceAmount {
		t.Errorf("Expected customer debit service amount to be %f, got %f", expected.CustomerDebitServiceAmount, response.CustomerDebitServiceAmount)
	}

	if response.DebitService != expected.DebitService {
		t.Errorf("Expected debit service to be %f, got %f", expected.DebitService, response.DebitService)
	}

	if response.EirMonthly != expected.EirMonthly {
		t.Errorf("Expected EIR monthly to be %f, got %f", expected.EirMonthly, response.EirMonthly)
	}

	if response.EirYearly != expected.EirYearly {
		t.Errorf("Expected EIR yearly to be %f, got %f", expected.EirYearly, response.EirYearly)
	}

	if response.EffectiveInterestRate != expected.EffectiveInterestRate {
		t.Errorf("Expected effective interest rate to be %f, got %f", expected.EffectiveInterestRate, response.EffectiveInterestRate)
	}

	if response.InstallmentAmount != expected.InstallmentAmount {
		t.Errorf("Expected installment amount to be %f, got %f", expected.InstallmentAmount, response.InstallmentAmount)
	}

	if response.InterestRate != expected.InterestRate {
		t.Errorf("Expected interest rate to be %f, got %f", expected.InterestRate, response.InterestRate)
	}

	if response.MdrAmount != expected.MdrAmount {
		t.Errorf("Expected MDR amount to be %f, got %f", expected.MdrAmount, response.MdrAmount)
	}

	if response.MerchantDebitServiceAmount != expected.MerchantDebitServiceAmount {
		t.Errorf("Expected merchant debit service amount to be %f, got %f", expected.MerchantDebitServiceAmount, response.MerchantDebitServiceAmount)
	}

	if response.MerchantTotalAmount != expected.MerchantTotalAmount {
		t.Errorf("Expected merchant total amount to be %f, got %f", expected.MerchantTotalAmount, response.MerchantTotalAmount)
	}

	if response.SettledToMerchant != expected.SettledToMerchant {
		t.Errorf("Expected settled to merchant to be %f, got %f", expected.SettledToMerchant, response.SettledToMerchant)
	}

	if response.TecMonthly != expected.TecMonthly {
		t.Errorf("Expected TEC monthly to be %f, got %f", expected.TecMonthly, response.TecMonthly)
	}

	if response.TecYearly != expected.TecYearly {
		t.Errorf("Expected TEC yearly to be %f, got %f", expected.TecYearly, response.TecYearly)
	}

	if response.TotalAmount != expected.TotalAmount {
		t.Errorf("Expected total amount to be %f, got %f", expected.TotalAmount, response.TotalAmount)
	}

	if response.TotalIof != expected.TotalIof {
		t.Errorf("Expected total IOF to be %f, got %f", expected.TotalIof, response.TotalIof)
	}

}

func downPaymentHelper(t *testing.T, plan *protos.DownPaymentResponses) {
	if len(plan.Responses) != 2 {
		t.Fatalf("Expected 2 responses, got %d", len(plan.Responses))
	}

	response := plan.Responses[0]
	if response.InstallmentAmount != 200 {
		t.Errorf("Expected installment amount to be 200, got %f", response.InstallmentAmount)
	}

	plans := response.Plans.Responses
	if len(plans) == 0 {
		t.Errorf("Expected at least one plan, got none")
	}

	firstPlan := plans[0]

	planDueDate := time.Date(2022, time.July, 20, 3, 0, 0, 0, time.UTC)
	actualDueDate := time.UnixMilli(firstPlan.DueDateMillis).UTC()
	if actualDueDate != planDueDate {
		t.Errorf("Expected first plan due date to be %s, got %s", planDueDate, actualDueDate)
	}

	response2 := plan.Responses[1]
	if response2.InstallmentAmount != 100 {
		t.Errorf("Expected installment amount to be 100, got %f", response2.InstallmentAmount)
	}

	plans2 := response2.Plans.Responses
	if len(plans2) == 0 {
		t.Errorf("Expected at least one plan, got none")
	}

	firstPlan2 := plans2[0]

	planDueDate2 := time.Date(2022, time.August, 20, 3, 0, 0, 0, time.UTC)
	actualDueDate2 := time.UnixMilli(firstPlan2.DueDateMillis).UTC()
	if actualDueDate2 != planDueDate2 {
		t.Errorf("Expected first plan due date to be %s, got %s", planDueDate2, actualDueDate2)
	}
}

func callBuff[I proto.Message, O proto.Message](i I, o O, arg ...string) error {

	param := i

	// Serialize PlanParams to bytes
	data, err := proto.Marshal(param)
	if err != nil {
		return err
	}

	cmd := exec.Command("../../target/release/payment-plan", arg...)

	// Create a bytes buffer to hold the serialized data
	var in bytes.Buffer
	var out bytes.Buffer
	var errOut bytes.Buffer
	in.Write(data)
	cmd.Stdin = &in
	cmd.Stdout = &out
	cmd.Stderr = &errOut

	// Execute the CLI command
	err = cmd.Run()
	if err != nil {
		return err
	}

	if errOut.Len() > 0 {
		errMsg := fmt.Sprintf("Error running payment-plan CLI: %s", errOut.String())
		err := errors.New(errMsg)
		return err
	}

	// Deserialize the response
	err = proto.Unmarshal(out.Bytes(), o)
	if err != nil {
		return err
	}

	return nil
}
