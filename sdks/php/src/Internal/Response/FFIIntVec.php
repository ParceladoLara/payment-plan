<?php
/* src/internal/Response/FFIIntVec.php */

namespace ParceladoLara\PaymentPlan\Internal\Response;


/**
 * @internal
 * Internal representation of Vec_int64_t for FFI.
 */
class FFIIntVec
{
  /** @var \FFI\CData Pointer to FFIInt (int64_t*) */
  public $ptr;
  /** @var int */
  public int $len;
  /** @var int */
  public int $cap;

  /**
   * Convert the FFIIntVec to an array of integers.
   * @return int[]
   */
  public function toArray(): array
  {
    $ints = [];
    if ($this->ptr === null || $this->len === 0) {
      return $ints;
    }

    for ($i = 0; $i < $this->len; $i++) {
      $cInt = $this->ptr[$i];
      $ints[] = (int)$cInt;
    }
    return $ints;
  }

  /**
   * Convert the FFIIntVec to an array of DateTimeImmutable objects.
   * @return \DateTimeImmutable[]
   */
  public function toDateTimeArray(): array
  {
    $dates = [];
    if ($this->ptr === null || $this->len === 0) {
      return $dates;
    }

    for ($i = 0; $i < $this->len; $i++) {
      $cInt = $this->ptr[$i];
      $cInt = (int)$cInt; // Ensure it's an integer
      $dates[] = new \DateTimeImmutable('@' . $cInt / 1000); // Convert milliseconds to seconds
    }
    return $dates;
  }
}
