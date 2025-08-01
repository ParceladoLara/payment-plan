<?php
/* src\response\Invoice.php */

namespace ParceladoLara\PaymentPlan\Response;


class Invoice
{
  public int $accumulatedDays;
  public float $factor;
  public float $accumulatedFactor;
  public \DateTimeImmutable $dueDate;
}
