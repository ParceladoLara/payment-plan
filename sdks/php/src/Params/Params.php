<?php
/* src/Params/Params.php */

namespace ParceladoLara\PaymentPlan\Params;

class Params
{
  public float $requestedAmount;
  public \DateTimeInterface $firstPaymentDate;
  public \DateTimeInterface $disbursementDate;
  public int $installments;
  public int $debitServicePercentage;
  public float $mdr;
  public float $tacPercentage;
  public float $iofOverall;
  public float $iofPercentage;
  public float $interestRate;
  public float $minInstallmentAmount;
  public float $maxTotalAmount;
  public bool $disbursementOnlyOnBusinessDays;

  public function __construct(
    float $requestedAmount,
    \DateTimeInterface $firstPaymentDate,
    \DateTimeInterface $disbursementDate,
    int $installments,
    int $debitServicePercentage,
    float $mdr,
    float $tacPercentage,
    float $iofOverall,
    float $iofPercentage,
    float $interestRate,
    float $minInstallmentAmount,
    float $maxTotalAmount,
    bool $disbursementOnlyOnBusinessDays
  ) {
    $this->requestedAmount = $requestedAmount;
    $this->firstPaymentDate = $firstPaymentDate;
    $this->disbursementDate = $disbursementDate;
    $this->installments = $installments;
    $this->debitServicePercentage = $debitServicePercentage;
    $this->mdr = $mdr;
    $this->tacPercentage = $tacPercentage;
    $this->iofOverall = $iofOverall;
    $this->iofPercentage = $iofPercentage;
    $this->interestRate = $interestRate;
    $this->minInstallmentAmount = $minInstallmentAmount;
    $this->maxTotalAmount = $maxTotalAmount;
    $this->disbursementOnlyOnBusinessDays = $disbursementOnlyOnBusinessDays;
  }
}
