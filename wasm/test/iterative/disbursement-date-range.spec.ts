import { test } from "node:test";
import * as assert from "node:assert";

import { disbursementDateRange } from "pkg/wasm_payment_plan";

test("disbursementDateRange", () => {
  const baseDate = new Date("2078-02-12");
  const days = 5;
  const result = disbursementDateRange(baseDate, days);
  const expected = [
    new Date("2078-02-16T06:00:00.000Z"),
    new Date("2078-02-22T06:00:00.000Z"),
  ];
  assert.deepStrictEqual(result, expected);
});
