import { test } from "node:test";
import * as assert from "node:assert";

import { getNonBusinessDaysBetween } from "pkg/wasm_payment_plan";

test("getNonBusinessDaysBetween", () => {
  const startDate = new Date("2078-11-12");
  const endDate = new Date("2078-11-22");
  const result = getNonBusinessDaysBetween(startDate, endDate);
  const expected = [
    new Date("2078-11-12T06:00:00.000Z"),
    new Date("2078-11-13T06:00:00.000Z"),
    new Date("2078-11-15T06:00:00.000Z"),
    new Date("2078-11-19T06:00:00.000Z"),
    new Date("2078-11-20T06:00:00.000Z"),
  ];
  assert.deepStrictEqual(result, expected);
});
