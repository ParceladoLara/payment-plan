import { test } from 'node:test';
import * as assert from 'node:assert';

import { getNonBusinessDaysBetween } from '../src/index.js';

test('getNonBusinessDaysBetween', () => {
  const startDate = new Date('2078-11-12');
  const endDate = new Date('2078-11-22');
  const result = getNonBusinessDaysBetween(startDate, endDate);
  const expected = [
    new Date('2078-11-12T03:00:00.000Z'),
    new Date('2078-11-13T03:00:00.000Z'),
    new Date('2078-11-15T03:00:00.000Z'),
    new Date('2078-11-19T03:00:00.000Z'),
    new Date('2078-11-20T03:00:00.000Z'),
  ];

  for (const date of result) {
    date.setUTCHours(3, 0, 0, 0); // Ensure the time is set to 03:00:00.000 UTC
  }
  assert.deepStrictEqual(result, expected);
});
