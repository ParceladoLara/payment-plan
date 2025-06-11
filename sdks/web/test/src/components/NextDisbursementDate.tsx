import { nextDisbursementDate } from '@parcelado_lara/payment-plan-wasm';
import React from 'react';

const NextDisbursementDate: React.FC = () => {
  const baseDate = new Date('2078-02-12');
  const result = nextDisbursementDate(baseDate);
  result.setUTCHours(3, 0, 0, 0); // Ensure the time is set to 03:00:00.000 UTC
  /*
    2078-02-12 = Saturday(invalid)
    2078-02-13 = Sunday(invalid)
    2078-02-14 = Bank holiday(invalid)
    2078-02-15 = Bank holiday(invalid)
    2078-02-16 = Wednesday(valid) 1
  */
  const str = JSON.stringify(result.toISOString(), null, 2);
  return <pre>{str}</pre>;
}

export default NextDisbursementDate;