import { getNonBusinessDaysBetween } from '@parcelado_lara/payment-plan-wasm';
import React from 'react';

const GetNonBusinessDaysBetween: React.FC = () => {
  const startDate = new Date('2078-11-12');
  const endDate = new Date('2078-11-22');
  const result = getNonBusinessDaysBetween(startDate, endDate);
  console.log(result); // [2078-11-12T03:00:00.000Z, 2078-11-13T03:00:00.000Z, 2078-11-15T03:00:00.000Z, 2078-11-19T03:00:00.000Z, 2078-11-20T03:00:00.000Z]

  /*
    2078-11-12 = Saturday(non-business)
    2078-11-13 = Sunday(non-business)
    2078-11-14 = Monday(business)
    2078-11-15 = Tuesday(non-business)
    2078-11-16 = Wednesday(business)
    2078-11-17 = Thursday(business)
    2078-11-18 = Friday(business)
    2078-11-19 = Saturday(non-business)
    2078-11-20 = Sunday(non-business)
    2078-11-21 = Monday(business)
    2078-11-22 = Tuesday(business)
    So the non-business days are 2078-11-12, 2078-11-13, 2078-11-15, 2078-11-19, and 2078-11-20.
  */
  const str = JSON.stringify(
    result.map(date => date.toISOString()),
    null,
    2
  );
  return <pre>{str}</pre>;
}

export default GetNonBusinessDaysBetween;