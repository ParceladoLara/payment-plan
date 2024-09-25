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

var param = protos.PlanParams{
	RequestedAmount:        7431,
	FirstPaymentDateMillis: time.Date(2024, time.October, 24, 0, 0, 0, 0, time.UTC).UnixMilli(),
	RequestedDateMillis:    time.Date(2024, time.February, 24, 0, 0, 0, 0, time.UTC).UnixMilli(),
	Installments:           18,
	DebitServicePercentage: 0,
	Mdr:                    0.05,
	TacPercentage:          0,
	IofOverall:             0.0038,
	IofPercentage:          0.03,
	InterestRate:           0.04,
	MinInstallmentAmount:   100,
	MaxTotalAmount:         100000,
}

var downPaymentParams = protos.DownPaymentParams{
	Params:                 &param,
	FirstPaymentDateMillis: time.Date(2022, time.June, 20, 0, 0, 0, 0, time.UTC).UnixMilli(),
	RequestedAmount:        200,
	Installments:           2,
	MinInstallmentAmount:   100,
}

func TestBMPPlan(t *testing.T) {
	var plan protos.PlanResponses

	err := callBuff(&param, &plan)
	if err != nil {
		t.Errorf("Error running payment-plan CLI: %s", err)
	}
}
func TestQiTechPlan(t *testing.T) {
	var plan protos.PlanResponses

	err := callBuff(&param, &plan)
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

	err := callBuff(&downPaymentParams, &plan, "-t", "down-payment")
	if err != nil {
		t.Fatalf("Error running payment-plan CLI: %s", err)
	}

	downPaymentHelper(t, &plan)
}

func downPaymentHelper(t *testing.T, plan *protos.DownPaymentResponses) {
	if len(plan.Responses) != 2 {
		t.Fatalf("Expected 2 responses, got %d", len(plan.Responses))
	}

	response := plan.Responses[0]
	if response.InstallmentAmount != 200 {
		t.Fatalf("Expected installment amount to be 200, got %f", response.InstallmentAmount)
	}

	plans := response.Plans.Responses
	if len(plans) == 0 {
		t.Fatalf("Expected at least one plan, got none")
	}

	firstPlan := plans[0]

	planDueDate := time.Date(2022, time.July, 20, 3, 0, 0, 0, time.UTC)
	actualDueDate := time.UnixMilli(firstPlan.DueDateMillis).UTC()
	if actualDueDate != planDueDate {
		t.Fatalf("Expected first plan due date to be %s, got %s", planDueDate, actualDueDate)
	}

	response2 := plan.Responses[1]
	if response2.InstallmentAmount != 100 {
		t.Fatalf("Expected installment amount to be 100, got %f", response2.InstallmentAmount)
	}

	plans2 := response2.Plans.Responses
	if len(plans2) == 0 {
		t.Fatalf("Expected at least one plan, got none")
	}

	firstPlan2 := plans2[0]

	planDueDate2 := time.Date(2022, time.August, 20, 3, 0, 0, 0, time.UTC)
	actualDueDate2 := time.UnixMilli(firstPlan2.DueDateMillis).UTC()
	if actualDueDate2 != planDueDate2 {
		t.Fatalf("Expected first plan due date to be %s, got %s", planDueDate2, actualDueDate2)
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
