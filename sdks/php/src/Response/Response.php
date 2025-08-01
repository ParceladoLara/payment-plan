<?php
/* src/response/Response.php */

namespace ParceladoLara\PaymentPlan\Response;


class Response
{
  public int $installment;
  public \DateTimeInterface $dueDate;
  public \DateTimeInterface $disbursementDate;
  public int $accumulatedDays;
  public float $daysIndex;
  public float $accumulatedDaysIndex;
  public float $interestRate;
  public float $installmentAmount;
  public float $installmentAmountWithoutTac;
  public float $totalAmount;
  public float $debitService;
  public float $customerDebitServiceAmount;
  public float $customerAmount;
  public float $calculationBasisForEffectiveInterestRate;
  public float $merchantDebitServiceAmount;
  public float $merchantTotalAmount;
  public float $settledToMerchant;
  public float $mdrAmount;
  public float $effectiveInterestRate;
  public float $totalEffectiveCost;
  public float $eirYearly;
  public float $tecYearly;
  public float $eirMonthly;
  public float $tecMonthly;
  public float $totalIof;
  public float $contractAmount;
  public float $contractAmountWithoutTac;
  public float $tacAmount;
  public float $iofPercentage;
  public float $overallIof;
  public float $preDisbursementAmount;
  public float $paidTotalIof;
  public float $paidContractAmount;
  /**
   * @var Invoice[]
   */
  public array $invoices;
}
