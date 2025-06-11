import { disbursementDateRange } from '@parcelado_lara/payment-plan-wasm';
import React from 'react';

const DisbursementDateRange: React.FC = () => {
  const baseDate = new Date('2078-02-12');
  const days = 5;
  const result = disbursementDateRange(baseDate, days);
  console.log(result); // [2078-02-16T03:00:00.000Z, 2078-02-22T03:00:00.000Z ]
  /*
    2078-02-12 = Saturday(invalid)
    2078-02-13 = Sunday(invalid)
    2078-02-14 = Bank holiday(invalid)
    2078-02-15 = Bank holiday(invalid)
    2078-02-16 = Wednesday(valid) 1
    2078-02-17 = Thursday(valid) 2
    2078-02-18 = Friday(valid) 3
    2078-02-19 = Saturday(invalid)
    2078-02-20 = Sunday(invalid)
    2078-02-21 = Tuesday(valid) 4
    2078-02-22 = Wednesday(valid) 5


    So the disbursement period is from 2078-02-16 to 2078-02-22.
  */
  const str = JSON.stringify(
    result.map(date => date.toISOString()),
    null,
    2
  );
  return <pre>{str}</pre>;
}

export default DisbursementDateRange;