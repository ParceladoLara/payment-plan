<?php
/* src/PaymentPlan.php */

namespace ParceladoLara\PaymentPlan;

use ParceladoLara\PaymentPlan\Internal\FFIPaymentPlan;
use ParceladoLara\PaymentPlan\Internal\Params\FFIParams;
use ParceladoLara\PaymentPlan\Internal\Params\FFIDownPaymentParams;

use ParceladoLara\PaymentPlan\Params\Params;
use ParceladoLara\PaymentPlan\Params\DownPaymentParams;

use ParceladoLara\PaymentPlan\Response\Response;
use ParceladoLara\PaymentPlan\Response\DownPaymentResponse;


class PaymentPlan
{
  /**
   * Calculates the payment plan using FFI.
   *
   * @param Params $params The parameters for the payment plan calculation.
   * @return Response[]
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function calculate(Params $params): array
  {
    $ffiParams = FFIParams::fromParams($params);
    return FFIPaymentPlan::calculatePaymentPlan($ffiParams);
  }

  /**
   * Calculates the down payment plan using FFI with Params object.
   * @param DownPaymentParams $params
   * @return DownPaymentResponse[]
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function calculateDownPayment(DownPaymentParams $params): array
  {
    $ffiParams = FFIDownPaymentParams::fromParams($params);
    return FFIPaymentPlan::calculateDownPaymentPlan($ffiParams);
  }


  /**
   * Retrieves the disbursement date range using FFI.
   *
   * @param DateTimeInterface $start Start date in Unix timestamp format.
   * @param int $days the number of days for the range.
   * @return DateTimeInterface[] Array of disbursement dates in Unix timestamp format.
   *
   * @throws \InvalidArgumentException if the days parameter is not positive.
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function disbursementDateRange(\DateTimeInterface $start, int $days): array
  {
    if ($days <= 0) {
      throw new \InvalidArgumentException("Days must be a positive integer");
    }
    // Convert DateTimeInterface to Unix timestamp
    $startTimestamp = $start->getTimestamp() * 1000; // Convert to milliseconds for FFI

    // Call the FFI method to get the disbursement date range
    $result = FFIPaymentPlan::disbursementDateRange($startTimestamp, $days);


    // Convert the result to an array of DateTimeInterface
    return array_map(fn($timestamp) => (new \DateTimeImmutable('@' . ($timestamp / 1000))), $result);
  }

  /**
   * Retrieves the next disbursement date using FFI.
   *
   * @param DateTimeInterface $baseDate The base date to calculate the next disbursement date from.
   * @return DateTimeInterface The next disbursement date.
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function nextDisbursementDate(\DateTimeInterface $baseDate): \DateTimeInterface
  {
    $baseTimestamp = $baseDate->getTimestamp() * 1000; // Convert to milliseconds for FFI
    $result = FFIPaymentPlan::nextDisbursementDate($baseTimestamp);
    return (new \DateTimeImmutable('@' . ($result / 1000))); // Convert back to DateTimeImmutable
  }

  /**
   * Retrieves the non-business days between two dates using FFI.
   *
   * @param \DateTimeInterface $start Start date.
   * @param \DateTimeInterface $end End date.
   * @return \DateTimeImmutable[] Array of non-business days.
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function getNonBusinessDaysBetween(
    \DateTimeInterface $start,
    \DateTimeInterface $end
  ): array {
    // Convert DateTimeInterface to Unix timestamp
    $startTimestamp = $start->getTimestamp() * 1000; // Convert to milliseconds for FFI
    $endTimestamp = $end->getTimestamp() * 1000; // Convert to milliseconds for FFI

    // Call the FFI method to get the non-business days
    return FFIPaymentPlan::getNonBusinessDaysBetween($startTimestamp, $endTimestamp);
  }
}
