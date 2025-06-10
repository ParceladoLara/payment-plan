import { test } from 'node:test';
import * as assert from 'node:assert';

import { disbursementDateRange } from '../src/index.js';

test('disbursementDateRange', () => {
  const baseDate = new Date('2078-02-12');
  const days = 5;
  const result = disbursementDateRange(baseDate, days);
  const expected = [
    new Date('2078-02-16T03:00:00.000Z'),
    new Date('2078-02-22T03:00:00.000Z'),
  ];
  for (const date of result) {
    date.setUTCHours(3, 0, 0, 0); // Ensure the time is set to 03:00:00.000 UTC
  }
  assert.deepStrictEqual(result, expected);
});
