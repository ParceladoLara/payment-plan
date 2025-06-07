import { test } from "node:test";
import * as assert from "node:assert";

import { disbursementDateRange } from "pkg/wasm_payment_plan";

test("disbursementDateRange", () => {
  const baseDate = new Date("2078-02-12");
  const days = 5;
  const result = disbursementDateRange(baseDate, days);
  const expected = [
    new Date("2078-02-16T00:00:00.000Z"),
    new Date("2078-02-22T00:00:00.000Z"),
  ];

  result.forEach((date, index) => {
    // Ensure the date is in UTC format
    date.setUTCHours(0, 0, 0, 0);
  });

  assert.deepStrictEqual(result, expected);
});
