<?php
/* src/internal/FFIPaymentPlan.php */

namespace ParceladoLara\PaymentPlan\Internal;


use ParceladoLara\PaymentPlan\Internal\Response\FFIResponseVec;
use ParceladoLara\PaymentPlan\Internal\Params\FFIParams;
use ParceladoLara\PaymentPlan\Internal\Params\FFIDownPaymentParams;
use ParceladoLara\PaymentPlan\Internal\Response\FFIDownPaymentResponseVec;
use ParceladoLara\PaymentPlan\Internal\Response\FFIIntArray;
use ParceladoLara\PaymentPlan\Internal\Response\FFIIntVec;

/**
 * @internal
 * This class is part of the internal API and should not be used directly.
 */
class FFIPaymentPlan
{

  private static ?\FFI $ffi = null;

  /**
   * Returns the path to the shared library depending on the OS.
   * @throws \RuntimeException if the OS is unsupported or the library is missing.
   */
  private static function getSharedLibraryPath(): string
  {
    $nativeDir = __DIR__ . '/native';
    switch (PHP_OS_FAMILY) {
      case 'Linux':
        $lib = $nativeDir . '/libpayment_plan.so';
        break;
      case 'Windows':
        $lib = $nativeDir . '/libpayment_plan.dll';
        break;
      default:
        throw new \RuntimeException('Unsupported OS for FFI Payment Plan');
    }
    if (!file_exists($lib)) {
      throw new \RuntimeException("Shared library not found: $lib");
    }
    return $lib;
  }

  /**
   * Returns the FFI instance, initializing it if necessary.
   * @throws \RuntimeException if the header file is missing.
   * @throws \RuntimeException if the OS is unsupported or the library is missing.
   */
  public static function getFFI(): \FFI
  {
    if (self::$ffi === null) {
      $headerPath = __DIR__ . '/native/payment_plan.h';
      if (!file_exists($headerPath)) {
        throw new \RuntimeException("Header file not found: $headerPath");
      }
      $header = file_get_contents($headerPath);
      self::$ffi = \FFI::cdef($header, self::getSharedLibraryPath());
    }
    return self::$ffi;
  }

  /**
   * Calculates the payment plan using FFI.
   *
   * @param FFIParams $params
   * @return \ParceladoLara\PaymentPlan\Response\Response[]
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function calculatePaymentPlan(FFIParams $params): array
  {

    $ffi = self::getFFI();

    // Allocate and populate the Params_t struct
    $cParams = $ffi->new('Params_t');
    $cParams = $params->toCData($cParams);

    // Allocate Vec_Response_t for output
    $cVecResponse = $ffi->new('Vec_Response_t');

    // Call the FFI function

    $result = $ffi->calculate_payment_plan($cParams, \FFI::addr($cVecResponse));
    if ($result !== 0) {
      throw new \RuntimeException("FFI calculate_payment_plan failed with code $result");
    }

    // Wrap the result in an FFIResponseVec object
    $responseVec = new FFIResponseVec();
    $responseVec->ptr = $cVecResponse->ptr;
    $responseVec->len = $cVecResponse->len;
    $responseVec->cap = $cVecResponse->cap;

    // Convert the response to an array
    $arr = [];
    try {
      $arr = $responseVec->toArray();
    } finally {
      // Free the allocated memory for the response vector
      // This is important to avoid memory leaks in FFI
      $ffi->free_response_vec($cVecResponse);
    }

    return $arr;
  }

  /**
   * Calculates the  down payment plan using FFI with Params object.
   *
   * @param FFIDownPaymentParams $params
   * @return \ParceladoLara\PaymentPlan\Response\DownPaymentResponse[]
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function calculateDownPaymentPlan(FFIDownPaymentParams $params): array
  {
    $ffi = self::getFFI();

    // Allocate and populate the DownPaymentParams_t struct
    $cDParams = $ffi->new('DownPaymentParams_t');
    $cDParams = $params->toCData($cDParams);
    if ($cDParams === null) {
      throw new \RuntimeException("Failed to create DownPaymentParams_t from FFIDownPaymentParams");
    }
    // Allocate Vec_DownPaymentResponse_t for output
    $cVecResponse = $ffi->new('Vec_DownPaymentResponse_t');

    // Call the FFI function
    $result = $ffi->calculate_down_payment_plan($cDParams, \FFI::addr($cVecResponse));
    if ($result !== 0) {
      throw new \RuntimeException("FFI calculate_down_payment_plan failed with code $result");
    }

    // Wrap the result in an FFIResponseVec object
    $responseVec = new FFIDownPaymentResponseVec();
    $responseVec->ptr = $cVecResponse->ptr;
    $responseVec->len = $cVecResponse->len;
    $responseVec->cap = $cVecResponse->cap;

    // Convert the response to an array
    $arr = [];
    try {
      $arr = $responseVec->toArray();
    } finally {
      // Free the allocated memory for the response vector
      // This is important to avoid memory leaks in FFI
      $ffi->free_down_payment_response_vec($cVecResponse);
    }

    return $arr;
  }

  /**
   * Retrieves the disbursement date range using FFI.
   *
   * @param int $start Start date in Unix timestamp format.
   * @param int $days the number of days for the range.
   * @return int[] Array of disbursement dates in Unix timestamp format.
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function disbursementDateRange(int $start, int $days): array
  {
    $ffi = self::getFFI();

    $cResult = $ffi->new('int64_2_array_t');
    $result = $ffi->disbursement_date_range($start, $days, \FFI::addr($cResult));

    if ($result !== 0) {
      throw new \RuntimeException("FFI disbursement_date_range failed with code $result");
    }

    // Convert the C data to a PHP representation
    $ffiIntArray = new FFIIntArray();
    $ffiIntArray->ptr = $cResult->idx;

    $arr = [];
    // Convert the FFIIntArray to a PHP array
    try {
      $arr = $ffiIntArray->toArray();
    } finally {
      //We don't need to free cResult as it's a simple struct
      //FFI does not require manual memory management for simple structs
    }
    return $arr;
  }


  /**
   * Retrieves the next disbursement date using FFI.
   * @param int $baseDate Base date in Unix timestamp format.
   * @return int Next disbursement date in Unix timestamp format.
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function nextDisbursementDate(int $baseDate): int
  {
    $ffi = self::getFFI();

    $result = $ffi->new('int64_t');
    $status = $ffi->next_disbursement_date($baseDate, \FFI::addr($result));

    if ($status !== 0) {
      throw new \RuntimeException("FFI next_disbursement_date failed with code $status");
    }


    return $result->cdata;
  }


  /**
   * Retrieves the non-business days between two dates using FFI.
   *
   * @param int $startDate Start date in Unix timestamp format.
   * @param int $endDate End date in Unix timestamp format.
   * @return \DateTimeImmutable[] Array of non-business days in Unix timestamp format.
   *
   * @throws \RuntimeException if the FFI call fails.
   * @throws \RuntimeException if the shared library or header file is missing.
   * @throws \RuntimeException if the OS is unsupported.
   */
  public static function getNonBusinessDaysBetween(int $startDate, int $endDate): array
  {
    $ffi = self::getFFI();

    $cResult = $ffi->new('Vec_int64_t');
    $result = $ffi->get_non_business_days_between($startDate, $endDate, \FFI::addr($cResult));

    if ($result !== 0) {
      throw new \RuntimeException("FFI get_non_business_days_between failed with code $result");
    }

    // Convert the C data to a PHP representation
    $ffiIntVec = new FFIIntVec();
    $ffiIntVec->ptr = $cResult->ptr;
    $ffiIntVec->len = $cResult->len;
    $ffiIntVec->cap = $cResult->cap;

    $arr = [];
    // Convert the FFIIntVec to a PHP array
    try {
      $arr = $ffiIntVec->toDateTimeArray();
    } finally {
      // Free the allocated memory for the response vector
      // This is important to avoid memory leaks in FFI
      $ffi->free_i64_vec($cResult);
    }

    return $arr;
  }
}
