import { test } from "node:test";
import * as assert from "node:assert";

import {  isBusinessDay } from "pkg/wasm_payment_plan";

test("isBusinessDay", () => {
  const date = new Date("2026-10-05");
  const result = isBusinessDay(date);
  const expected = true;

  assert.strictEqual(result.valueOf(), expected);
});

test("isBusinessDay on a weekend", () => {
  const date = new Date("2026-10-04");
  const result = isBusinessDay(date);
  const expected = false;

  assert.strictEqual(result.valueOf(), expected);
});

test("isBusinessDay on a holiday", () => {
  const date = new Date("2026-12-25");
  const result = isBusinessDay(date);
  const expected = false;

  assert.strictEqual(result.valueOf(), expected);
});