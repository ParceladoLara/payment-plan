<?php
/* src/internal/Response/FFIIntArray.php */

namespace ParceladoLara\PaymentPlan\Internal\Response;

/**
 * @internal
 * FFIIntArray is an internal representation of an array of integers for FFI.
 * It provides a method to convert the FFI array to a PHP array.
 */
class FFIIntArray
{
  /** @var \FFI\CData Pointer to an array of integers */
  public $ptr;

  /**
   * Convert the FFIIntArray to a PHP array.
   * @return int[]
   */
  public function toArray(): array
  {
    $array = [];

    // Check if the pointer is valid
    if ($this->ptr === null) {
      return $array; // Return an empty array if the pointer is null
    }
    // Iterate through the FFI array and convert it to a PHP array
    for ($i = 0; $i < 2; $i++) {
      // Check if the pointer is valid before accessing it
      $value = $this->ptr[$i] ?? null;
      if ($value !== null) {
        $array[] = $value->cdata; // Add the value to the PHP array
      }
    }


    return $array;
  }
}
