import { test } from "node:test";

import { nextDisbursementDate } from "pkg/wasm_payment_plan";

test("next disbursement date test", () => {
  const result = nextDisbursementDate(new Date("2087-03-03"));
});
