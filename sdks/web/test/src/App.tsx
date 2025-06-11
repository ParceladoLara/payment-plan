import React, { useState } from 'react';
import CalculateDownPaymentPlan from './components/CalculateDownPaymentPlan';
import NextDisbursementDate from './components/NextDisbursementDate';
import CalculatePaymentPlan from './components/CalculatePaymentPlan';
import DisbursementDateRange from './components/DisbursementDateRange';
import GetNonBusinessDaysBetween from './components/GetNonBusinessDaysBetween';

type ComponentKey =
  | 'calculateDownPaymentPlan'
  | 'nextDisbursementDate'
  | 'calculatePaymentPlan'
  | 'disbursementDateRange'
  | 'getNonBusinessDaysBetween';

const components: Record<ComponentKey, React.FC> = {
  calculateDownPaymentPlan: CalculateDownPaymentPlan,
  nextDisbursementDate: NextDisbursementDate,
  calculatePaymentPlan: CalculatePaymentPlan,
  disbursementDateRange: DisbursementDateRange,
  getNonBusinessDaysBetween: GetNonBusinessDaysBetween,
};

const App: React.FC = () => {
  const [selected, setSelected] = useState<ComponentKey>('calculateDownPaymentPlan');
  const SelectedComponent = components[selected];

  return (
    <div>
      <header style={{ display: 'flex', gap: 8, padding: 16, background: '#eee' }}>
        <button onClick={() => setSelected('calculateDownPaymentPlan')}>Calculate Down Payment Plan</button>
        <button onClick={() => setSelected('nextDisbursementDate')}>Next Disbursement Date</button>
        <button onClick={() => setSelected('calculatePaymentPlan')}>Calculate Payment Plan</button>
        <button onClick={() => setSelected('disbursementDateRange')}>Disbursement Date Range</button>
        <button onClick={() => setSelected('getNonBusinessDaysBetween')}>Get Non Business Days Between</button>
      </header>
      <main style={{ padding: 16 }}>
        <SelectedComponent />
      </main>
    </div>
  );
};

export default App;