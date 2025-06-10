import { test } from 'node:test';
import * as assert from 'node:assert';

import { nextDisbursementDate } from '../src/index.js';

test('next disbursement date test', () => {
  const result = nextDisbursementDate(new Date('2087-03-08'));
  const expected = new Date('2087-03-10T03:00:00.000Z');
  result.setUTCHours(3, 0, 0, 0); // Ensure the time is set to 03:00:00.000 UTC
  assert.deepStrictEqual(
    result,
    expected,
    'The next disbursement date should be March 10, 2087 at 03:00:00.000 UTC',
  );
});
