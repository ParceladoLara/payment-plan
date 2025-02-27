import { test } from "node:test";
import * as assert from "node:assert";

import { nextDisbursementDate } from "pkg/wasm_payment_plan";

test("next disbursement date test", () => {
  const result = nextDisbursementDate(new Date("2025-03-03"));
});
