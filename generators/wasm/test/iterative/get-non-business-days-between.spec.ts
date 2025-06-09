import { test } from "node:test";
import * as assert from "node:assert";

import { getNonBusinessDaysBetween } from "pkg/wasm_payment_plan";

test("getNonBusinessDaysBetween", () => {
  const startDate = new Date("2078-11-12");
  const endDate = new Date("2078-11-22");
  const result = getNonBusinessDaysBetween(startDate, endDate);
  const expected = [
    new Date("2078-11-12T00:00:00.000Z"),
    new Date("2078-11-13T00:00:00.000Z"),
    new Date("2078-11-15T00:00:00.000Z"),
    new Date("2078-11-19T00:00:00.000Z"),
    new Date("2078-11-20T00:00:00.000Z"),
  ];

  result.forEach((date, index) => {
    // Ensure the date is in UTC format
    date.setUTCHours(0, 0, 0, 0);
  });
  assert.deepStrictEqual(result, expected);
});
