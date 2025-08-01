<?php
/* src/Params/DownPaymentParams.php */

namespace ParceladoLara\PaymentPlan\Params;


class DownPaymentParams
{
  public float $requestedAmount;
  public float $minInstallmentAmount;
  public \DateTimeInterface $firstPaymentDate;
  public int $installments;
  public Params $params;

  public function __construct(
    float $requestedAmount,
    float $minInstallmentAmount,
    \DateTimeInterface $firstPaymentDate,
    int $installments,
    Params $params
  ) {
    $this->params = $params;
    $this->requestedAmount = $requestedAmount;
    $this->minInstallmentAmount = $minInstallmentAmount;
    $this->firstPaymentDate = $firstPaymentDate;
    $this->installments = $installments;
  }
}
