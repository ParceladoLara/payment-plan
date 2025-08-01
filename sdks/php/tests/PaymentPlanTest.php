<?php

declare(strict_types=1);

namespace ParceladoLara\PaymentPlan\Tests;

use PHPUnit\Framework\TestCase;
use ParceladoLara\PaymentPlan\Params\Params;
use ParceladoLara\PaymentPlan\Params\DownPaymentParams;
use ParceladoLara\PaymentPlan\PaymentPlan;
use ParceladoLara\PaymentPlan\Response\Response;
use ParceladoLara\PaymentPlan\Response\Invoice;
use ParceladoLara\PaymentPlan\Response\DownPaymentResponse;
use DateTimeImmutable;

class PaymentPlanTest extends TestCase
{
  public function testCalculateReturnsArrayOfResponses(): void
  {

    $expectedInvoice1 = new Invoice();
    $expectedInvoice1->accumulatedDays = 28;
    $expectedInvoice1->factor = 0.981371965896169;
    $expectedInvoice1->accumulatedFactor = 0.981371965896169;
    $expectedInvoice1->dueDate = new DateTimeImmutable('2025-05-05');

    $expectedInvoice2 = new Invoice();
    $expectedInvoice2->accumulatedDays = 57;
    $expectedInvoice2->factor = 0.958839243657051;
    $expectedInvoice2->accumulatedFactor = 1.94021120955322;
    $expectedInvoice2->dueDate = new DateTimeImmutable('2025-06-03');

    $expectedInvoice3 = new Invoice();
    $expectedInvoice3->accumulatedDays = 87;
    $expectedInvoice3->factor = 0.936823882407599;
    $expectedInvoice3->accumulatedFactor = 2.8770350919608187;
    $expectedInvoice3->dueDate = new DateTimeImmutable('2025-07-03');

    $expectedInvoice4 = new Invoice();
    $expectedInvoice4->accumulatedDays = 119;
    $expectedInvoice4->factor = 0.914302133077605;
    $expectedInvoice4->accumulatedFactor = 3.791337225038424;
    $expectedInvoice4->dueDate = new DateTimeImmutable('2025-08-04');

    $expected1 = new Response();
    $expected1->installment = 1;
    $expected1->dueDate = new DateTimeImmutable('2025-05-05');
    $expected1->disbursementDate = new DateTimeImmutable('2025-04-07');
    $expected1->accumulatedDays = 28;
    $expected1->daysIndex = 0.981371965896169;
    $expected1->accumulatedDaysIndex = 0.981371965896169;
    $expected1->interestRate = 0.0235;
    $expected1->installmentAmount = 7996.8;
    $expected1->installmentAmountWithoutTac = 0.0;
    $expected1->totalAmount = 7996.8;
    $expected1->debitService = 148.96000000000018;
    $expected1->customerDebitServiceAmount = 148.96000000000018;
    $expected1->customerAmount = 7996.8;
    $expected1->calculationBasisForEffectiveInterestRate = 7948.96;
    $expected1->merchantDebitServiceAmount = 0.0;
    $expected1->merchantTotalAmount = 390.0;
    $expected1->settledToMerchant = 7410.0;
    $expected1->mdrAmount = 390.0;
    $expected1->effectiveInterestRate = 0.0206;
    $expected1->totalEffectiveCost = 0.0274;
    $expected1->eirYearly = 0.277782;
    $expected1->tecYearly = 0.383782;
    $expected1->eirMonthly = 0.0206;
    $expected1->tecMonthly = 0.0274;
    $expected1->totalIof = 47.84;
    $expected1->contractAmount = 7847.84;
    $expected1->contractAmountWithoutTac = 0.0;
    $expected1->tacAmount = 0.0;
    $expected1->iofPercentage = 8.2e-5;
    $expected1->overallIof = 0.0038;
    $expected1->preDisbursementAmount = 7800.0;
    $expected1->paidTotalIof = 47.84;
    $expected1->paidContractAmount = 7847.84;
    $expected1->invoices = [$expectedInvoice1];

    $expected2 = new Response();
    $expected2->installment = 2;
    $expected2->dueDate = new DateTimeImmutable('2025-06-03');
    $expected2->disbursementDate = new DateTimeImmutable('2025-04-07');
    $expected2->accumulatedDays = 57;
    $expected2->daysIndex = 0.958839243657051;
    $expected2->accumulatedDaysIndex = 1.94021120955322;
    $expected2->interestRate = 0.0235;
    $expected2->installmentAmount = 4049.72;
    $expected2->installmentAmountWithoutTac = 0.0;
    $expected2->totalAmount = 8099.44;
    $expected2->debitService = 242.1299999999996;
    $expected2->customerDebitServiceAmount = 242.1299999999996;
    $expected2->customerAmount = 4049.72;
    $expected2->calculationBasisForEffectiveInterestRate = 4021.0649999999996;
    $expected2->merchantDebitServiceAmount = 0.0;
    $expected2->merchantTotalAmount = 390.0;
    $expected2->settledToMerchant = 7410.0;
    $expected2->mdrAmount = 390.0;
    $expected2->effectiveInterestRate = 0.022;
    $expected2->totalEffectiveCost = 0.0274;
    $expected2->eirYearly = 0.298378;
    $expected2->tecYearly = 0.382981;
    $expected2->eirMonthly = 0.022;
    $expected2->tecMonthly = 0.0274;
    $expected2->totalIof = 57.31;
    $expected2->contractAmount = 7857.31;
    $expected2->contractAmountWithoutTac = 0.0;
    $expected2->tacAmount = 0.0;
    $expected2->iofPercentage = 8.2e-5;
    $expected2->overallIof = 0.0038;
    $expected2->preDisbursementAmount = 7800.0;
    $expected2->paidTotalIof = 57.31;
    $expected2->paidContractAmount = 7857.31;
    $expected2->invoices = [$expectedInvoice1, $expectedInvoice2];

    $expected3 = new Response();
    $expected3->installment = 3;
    $expected3->dueDate = new DateTimeImmutable('2025-07-03');
    $expected3->disbursementDate = new DateTimeImmutable('2025-04-07');
    $expected3->accumulatedDays = 87;
    $expected3->daysIndex = 0.936823882407599;
    $expected3->accumulatedDaysIndex = 2.8770350919608187;
    $expected3->interestRate = 0.0235;
    $expected3->installmentAmount = 2734.44;
    $expected3->installmentAmountWithoutTac = 0.0;
    $expected3->totalAmount = 8203.32;
    $expected3->debitService = 336.2299999999997;
    $expected3->customerDebitServiceAmount = 336.2299999999997;
    $expected3->customerAmount = 2734.44;
    $expected3->calculationBasisForEffectiveInterestRate = 2712.0766666666664;
    $expected3->merchantDebitServiceAmount = 0.0;
    $expected3->merchantTotalAmount = 390.0;
    $expected3->settledToMerchant = 7410.0;
    $expected3->mdrAmount = 390.0;
    $expected3->effectiveInterestRate = 0.0225;
    $expected3->totalEffectiveCost = 0.0272;
    $expected3->eirYearly = 0.306592;
    $expected3->tecYearly = 0.380434;
    $expected3->eirMonthly = 0.0225;
    $expected3->tecMonthly = 0.0272;
    $expected3->totalIof = 67.09;
    $expected3->contractAmount = 7867.09;
    $expected3->contractAmountWithoutTac = 0.0;
    $expected3->tacAmount = 0.0;
    $expected3->iofPercentage = 8.2e-5;
    $expected3->overallIof = 0.0038;
    $expected3->preDisbursementAmount = 7799.99;
    $expected3->paidTotalIof = 67.08;
    $expected3->paidContractAmount = 7867.08;
    $expected3->invoices = [$expectedInvoice1, $expectedInvoice2, $expectedInvoice3];

    $expected4 = new Response();
    $expected4->installment = 4;
    $expected4->dueDate = new DateTimeImmutable('2025-08-04');
    $expected4->disbursementDate = new DateTimeImmutable('2025-04-07');
    $expected4->accumulatedDays = 119;
    $expected4->daysIndex = 0.914302133077605;
    $expected4->accumulatedDaysIndex = 3.791337225038424;
    $expected4->interestRate = 0.0235;
    $expected4->installmentAmount = 2077.73;
    $expected4->installmentAmountWithoutTac = 0.0;
    $expected4->totalAmount = 8310.92;
    $expected4->debitService = 433.56000000000006;
    $expected4->customerDebitServiceAmount = 433.56000000000006;
    $expected4->customerAmount = 2077.73;
    $expected4->calculationBasisForEffectiveInterestRate = 2058.39;
    $expected4->merchantDebitServiceAmount = 0.0;
    $expected4->merchantTotalAmount = 390.0;
    $expected4->settledToMerchant = 7410.0;
    $expected4->mdrAmount = 390.0;
    $expected4->effectiveInterestRate = 0.0228;
    $expected4->totalEffectiveCost = 0.0271;
    $expected4->eirYearly = 0.310455;
    $expected4->tecYearly = 0.377876;
    $expected4->eirMonthly = 0.0228;
    $expected4->tecMonthly = 0.0271;
    $expected4->totalIof = 77.36;
    $expected4->contractAmount = 7877.36;
    $expected4->contractAmountWithoutTac = 0.0;
    $expected4->tacAmount = 0.0;
    $expected4->iofPercentage = 8.2e-5;
    $expected4->overallIof = 0.0038;
    $expected4->preDisbursementAmount = 7800.02;
    $expected4->paidTotalIof = 77.38;
    $expected4->paidContractAmount = 7877.38;
    $expected4->invoices = [$expectedInvoice1, $expectedInvoice2, $expectedInvoice3, $expectedInvoice4];



    $params = new Params(
      7800.0, // requested_amount
      new DateTimeImmutable('2025-05-03 05:00:00'), // first_payment_date
      new DateTimeImmutable('2025-04-05 05:00:00'), // disbursementDate
      4,     // installments
      0,      // debitService_percentage
      0.05,   // mdr
      0.0,    // tac_percentage
      0.0038, // iof_overall
      0.000082, // iofPercentage
      0.0235, // interestRate
      100.0,   // min_installmentAmount
      1000000, // max_totalAmount
      true    // disbursement_only_on_business_days
    );

    $result = PaymentPlan::calculate($params);

    $this->assertIsArray($result);
    $this->assertNotEmpty($result);
    $this->assertIsObject($result[0]);
    $this->assertEquals($expected1, $result[0]);
    $this->assertEquals($expected2, $result[1]);
    $this->assertEquals($expected3, $result[2]);
    $this->assertEquals($expected4, $result[3]);
  }


  public function testCalculateDownPaymentReturnsArrayOfResponses(): void
  {


    $expectedInvoice1_1 = new Invoice();
    $expectedInvoice1_1->accumulatedDays = 25;
    $expectedInvoice1_1->factor = 0.981371965896169;
    $expectedInvoice1_1->accumulatedFactor = 0.981371965896169;
    $expectedInvoice1_1->dueDate = new DateTimeImmutable('2025-06-03');

    $expectedInvoice1_2 = new Invoice();
    $expectedInvoice1_2->accumulatedDays = 55;
    $expectedInvoice1_2->factor = 0.958839243657051;
    $expectedInvoice1_2->accumulatedFactor = 1.94021120955322;
    $expectedInvoice1_2->dueDate = new DateTimeImmutable('2025-07-03');

    $expectedInvoice1_3 = new Invoice();
    $expectedInvoice1_3->accumulatedDays = 87;
    $expectedInvoice1_3->factor = 0.935788233217493;
    $expectedInvoice1_3->accumulatedFactor = 2.875999442770713;
    $expectedInvoice1_3->dueDate = new DateTimeImmutable('2025-08-04');

    $expectedInvoice1_4 = new Invoice();
    $expectedInvoice1_4->accumulatedDays = 117;
    $expectedInvoice1_4->factor = 0.913291381450307;
    $expectedInvoice1_4->accumulatedFactor = 3.78929082422102;
    $expectedInvoice1_4->dueDate = new DateTimeImmutable('2025-09-03');



    $expectedInvoice2_1 = new Invoice();
    $expectedInvoice2_1->accumulatedDays = 24;
    $expectedInvoice2_1->factor = 0.981371965896169;
    $expectedInvoice2_1->accumulatedFactor = 0.981371965896169;
    $expectedInvoice2_1->dueDate = new DateTimeImmutable('2025-07-03');

    $expectedInvoice2_2 = new Invoice();
    $expectedInvoice2_2->accumulatedDays = 56;
    $expectedInvoice2_2->factor = 0.957779256710963;
    $expectedInvoice2_2->accumulatedFactor = 1.9391512226071321;
    $expectedInvoice2_2->dueDate = new DateTimeImmutable('2025-08-04');

    $expectedInvoice2_3 = new Invoice();
    $expectedInvoice2_3->accumulatedDays = 86;
    $expectedInvoice2_3->factor = 0.93475372892694;
    $expectedInvoice2_3->accumulatedFactor = 2.873904951534072;
    $expectedInvoice2_3->dueDate = new DateTimeImmutable('2025-09-03');

    $expectedInvoice2_4 = new Invoice();
    $expectedInvoice2_4->accumulatedDays = 116;
    $expectedInvoice2_4->factor = 0.912281747198563;
    $expectedInvoice2_4->accumulatedFactor = 3.786186698732635;
    $expectedInvoice2_4->dueDate = new DateTimeImmutable('2025-10-03');



    $expectedInvoice3_1 = new Invoice();
    $expectedInvoice3_1->accumulatedDays = 26;
    $expectedInvoice3_1->factor = 0.980287069256833;
    $expectedInvoice3_1->accumulatedFactor = 0.980287069256833;
    $expectedInvoice3_1->dueDate = new DateTimeImmutable('2025-08-04');

    $expectedInvoice3_2 = new Invoice();
    $expectedInvoice3_2->accumulatedDays = 56;
    $expectedInvoice3_2->factor = 0.956720441569568;
    $expectedInvoice3_2->accumulatedFactor = 1.937007510826401;
    $expectedInvoice3_2->dueDate = new DateTimeImmutable('2025-09-03');

    $expectedInvoice3_3 = new Invoice();
    $expectedInvoice3_3->accumulatedDays = 86;
    $expectedInvoice3_3->factor = 0.933720368270266;
    $expectedInvoice3_3->accumulatedFactor = 2.870727879096667;
    $expectedInvoice3_3->dueDate = new DateTimeImmutable('2025-10-03');

    $expectedInvoice3_4 = new Invoice();
    $expectedInvoice3_4->accumulatedDays = 117;
    $expectedInvoice3_4->factor = 0.912281747198563;
    $expectedInvoice3_4->accumulatedFactor = 3.78300962629523;
    $expectedInvoice3_4->dueDate = new DateTimeImmutable('2025-11-03');



    $expectedInvoice4_1 = new Invoice();
    $expectedInvoice4_1->accumulatedDays = 23;
    $expectedInvoice4_1->factor = 0.981371965896169;
    $expectedInvoice4_1->accumulatedFactor = 0.981371965896169;
    $expectedInvoice4_1->dueDate = new DateTimeImmutable('2025-09-03');

    $expectedInvoice4_2 = new Invoice();
    $expectedInvoice4_2->accumulatedDays = 53;
    $expectedInvoice4_2->factor = 0.957779256710963;
    $expectedInvoice4_2->accumulatedFactor = 1.9391512226071321;
    $expectedInvoice4_2->dueDate = new DateTimeImmutable('2025-10-03');

    $expectedInvoice4_3 = new Invoice();
    $expectedInvoice4_3->accumulatedDays = 84;
    $expectedInvoice4_3->factor = 0.935788233217493;
    $expectedInvoice4_3->accumulatedFactor = 2.874939455824625;
    $expectedInvoice4_3->dueDate = new DateTimeImmutable('2025-11-03');

    $expectedInvoice4_4 = new Invoice();
    $expectedInvoice4_4->accumulatedDays = 114;
    $expectedInvoice4_4->factor = 0.914302133077605;
    $expectedInvoice4_4->accumulatedFactor = 3.78924158890223;
    $expectedInvoice4_4->dueDate = new DateTimeImmutable('2025-12-03');


    $expected1_1 = new Response();
    $expected1_1->installment = 1;
    $expected1_1->dueDate = new DateTimeImmutable('2025-06-03');
    $expected1_1->disbursementDate = new DateTimeImmutable('2025-05-09');
    $expected1_1->accumulatedDays = 25;
    $expected1_1->daysIndex = 0.981371965896169;
    $expected1_1->accumulatedDaysIndex = 0.981371965896169;
    $expected1_1->interestRate = 0.0235;
    $expected1_1->installmentAmount = 7994.82;
    $expected1_1->installmentAmountWithoutTac = 0.0;
    $expected1_1->totalAmount = 7994.82;
    $expected1_1->debitService = 148.92999999999972;
    $expected1_1->customerDebitServiceAmount = 148.92999999999972;
    $expected1_1->customerAmount = 7994.82;
    $expected1_1->calculationBasisForEffectiveInterestRate = 7948.929999999999;
    $expected1_1->merchantDebitServiceAmount = 0.0;
    $expected1_1->merchantTotalAmount = 390.0;
    $expected1_1->settledToMerchant = 7410.0;
    $expected1_1->mdrAmount = 390.0;
    $expected1_1->effectiveInterestRate = 0.0231;
    $expected1_1->totalEffectiveCost = 0.0305;
    $expected1_1->eirYearly = 0.315926;
    $expected1_1->tecYearly = 0.433592;
    $expected1_1->eirMonthly = 0.0231;
    $expected1_1->tecMonthly = 0.0305;
    $expected1_1->totalIof = 45.89;
    $expected1_1->contractAmount = 7845.89;
    $expected1_1->contractAmountWithoutTac = 0.0;
    $expected1_1->tacAmount = 0.0;
    $expected1_1->iofPercentage = 0.000082;
    $expected1_1->overallIof = 0.0038;
    $expected1_1->preDisbursementAmount = 7800.0;
    $expected1_1->paidTotalIof = 45.89;
    $expected1_1->paidContractAmount = 7845.89;
    $expected1_1->invoices = [$expectedInvoice1_1];

    $expected1_2 = new Response();
    $expected1_2->installment = 2;
    $expected1_2->dueDate = new DateTimeImmutable('2025-07-03');
    $expected1_2->disbursementDate = new DateTimeImmutable('2025-05-09');
    $expected1_2->accumulatedDays = 55;
    $expected1_2->daysIndex = 0.958839243657051;
    $expected1_2->accumulatedDaysIndex = 1.94021120955322;
    $expected1_2->interestRate = 0.0235;
    $expected1_2->installmentAmount = 4048.88;
    $expected1_2->installmentAmountWithoutTac = 0.0;
    $expected1_2->totalAmount = 8097.76;
    $expected1_2->debitService = 242.07000000000022;
    $expected1_2->customerDebitServiceAmount = 242.07000000000022;
    $expected1_2->customerAmount = 4048.88;
    $expected1_2->calculationBasisForEffectiveInterestRate = 4021.0350000000003;
    $expected1_2->merchantDebitServiceAmount = 0.0;
    $expected1_2->merchantTotalAmount = 390.0;
    $expected1_2->settledToMerchant = 7410.0;
    $expected1_2->mdrAmount = 390.0;
    $expected1_2->effectiveInterestRate = 0.0234;
    $expected1_2->totalEffectiveCost = 0.029;
    $expected1_2->eirYearly = 0.319877;
    $expected1_2->tecYearly = 0.408833;
    $expected1_2->eirMonthly = 0.0234;
    $expected1_2->tecMonthly = 0.029;
    $expected1_2->totalIof = 55.69;
    $expected1_2->contractAmount = 7855.69;
    $expected1_2->contractAmountWithoutTac = 0.0;
    $expected1_2->tacAmount = 0.0;
    $expected1_2->iofPercentage = 0.000082;
    $expected1_2->overallIof = 0.0038;
    $expected1_2->preDisbursementAmount = 7799.99;
    $expected1_2->paidTotalIof = 55.68;
    $expected1_2->paidContractAmount = 7855.68;
    $expected1_2->invoices = [$expectedInvoice1_1, $expectedInvoice1_2];


    $expected1_3 = new Response();
    $expected1_3->installment = 3;
    $expected1_3->dueDate = new DateTimeImmutable('2025-08-04');
    $expected1_3->disbursementDate = new DateTimeImmutable('2025-05-09');
    $expected1_3->accumulatedDays = 87;
    $expected1_3->daysIndex = 0.935788233217493;
    $expected1_3->accumulatedDaysIndex = 2.875999442770713;
    $expected1_3->interestRate = 0.0235;
    $expected1_3->installmentAmount = 2735.05;
    $expected1_3->installmentAmountWithoutTac = 0.0;
    $expected1_3->totalAmount = 8205.15;
    $expected1_3->debitService = 339.13999999999965;
    $expected1_3->customerDebitServiceAmount = 339.13999999999965;
    $expected1_3->customerAmount = 2735.05;
    $expected1_3->calculationBasisForEffectiveInterestRate = 2713.0466666666666;
    $expected1_3->merchantDebitServiceAmount = 0.0;
    $expected1_3->merchantTotalAmount = 390.0;
    $expected1_3->settledToMerchant = 7410.0;
    $expected1_3->mdrAmount = 390.0;
    $expected1_3->effectiveInterestRate = 0.0234;
    $expected1_3->totalEffectiveCost = 0.0282;
    $expected1_3->eirYearly = 0.320481;
    $expected1_3->tecYearly = 0.396244;
    $expected1_3->eirMonthly = 0.0234;
    $expected1_3->tecMonthly = 0.0282;
    $expected1_3->totalIof = 66.01;
    $expected1_3->contractAmount = 7866.01;
    $expected1_3->contractAmountWithoutTac = 0.0;
    $expected1_3->tacAmount = 0.0;
    $expected1_3->iofPercentage = 0.000082;
    $expected1_3->overallIof = 0.0038;
    $expected1_3->preDisbursementAmount = 7799.99;
    $expected1_3->paidTotalIof = 66.0;
    $expected1_3->paidContractAmount = 7866.0;
    $expected1_3->invoices = [$expectedInvoice1_1, $expectedInvoice1_2, $expectedInvoice1_3];


    $expected1_4 = new Response();
    $expected1_4->installment = 4;
    $expected1_4->dueDate = new DateTimeImmutable('2025-09-03');
    $expected1_4->disbursementDate = new DateTimeImmutable('2025-05-09');
    $expected1_4->accumulatedDays = 117;
    $expected1_4->daysIndex = 0.913291381450307;
    $expected1_4->accumulatedDaysIndex = 3.78929082422102;
    $expected1_4->interestRate = 0.0235;
    $expected1_4->installmentAmount = 2078.54;
    $expected1_4->installmentAmountWithoutTac = 0.0;
    $expected1_4->totalAmount = 8314.16;
    $expected1_4->debitService = 437.9499999999999;
    $expected1_4->customerDebitServiceAmount = 437.9499999999999;
    $expected1_4->customerAmount = 2078.54;
    $expected1_4->calculationBasisForEffectiveInterestRate = 2059.4875;
    $expected1_4->merchantDebitServiceAmount = 0.0;
    $expected1_4->merchantTotalAmount = 390.0;
    $expected1_4->settledToMerchant = 7410.0;
    $expected1_4->mdrAmount = 390.0;
    $expected1_4->effectiveInterestRate = 0.0236;
    $expected1_4->totalEffectiveCost = 0.0279;
    $expected1_4->eirYearly = 0.323112;
    $expected1_4->tecYearly = 0.391907;
    $expected1_4->eirMonthly = 0.0236;
    $expected1_4->tecMonthly = 0.0279;
    $expected1_4->totalIof = 76.21;
    $expected1_4->contractAmount = 7876.21;
    $expected1_4->contractAmountWithoutTac = 0.0;
    $expected1_4->tacAmount = 0.0;
    $expected1_4->iofPercentage = 0.000082;
    $expected1_4->overallIof = 0.0038;
    $expected1_4->preDisbursementAmount = 7799.98;
    $expected1_4->paidTotalIof = 76.19;
    $expected1_4->paidContractAmount = 7876.19;
    $expected1_4->invoices = [$expectedInvoice1_1, $expectedInvoice1_2, $expectedInvoice1_3, $expectedInvoice1_4];



    $expected2_1 = new Response();
    $expected2_1->installment = 1;
    $expected2_1->dueDate = new DateTimeImmutable('2025-07-03');
    $expected2_1->disbursementDate = new DateTimeImmutable('2025-06-09');
    $expected2_1->accumulatedDays = 24;
    $expected2_1->daysIndex = 0.981371965896169;
    $expected2_1->accumulatedDaysIndex = 0.981371965896169;
    $expected2_1->interestRate = 0.0235;
    $expected2_1->installmentAmount = 7994.17;
    $expected2_1->installmentAmountWithoutTac = 0.0;
    $expected2_1->totalAmount = 7994.17;
    $expected2_1->debitService = 148.92000000000007;
    $expected2_1->customerDebitServiceAmount = 148.92000000000007;
    $expected2_1->customerAmount = 7994.17;
    $expected2_1->calculationBasisForEffectiveInterestRate = 7948.92;
    $expected2_1->merchantDebitServiceAmount = 0.0;
    $expected2_1->merchantTotalAmount = 390.0;
    $expected2_1->settledToMerchant = 7410.0;
    $expected2_1->mdrAmount = 390.0;
    $expected2_1->effectiveInterestRate = 0.0241;
    $expected2_1->totalEffectiveCost = 0.0317;
    $expected2_1->eirYearly = 0.331065;
    $expected2_1->tecYearly = 0.453471;
    $expected2_1->eirMonthly = 0.0241;
    $expected2_1->tecMonthly = 0.0317;
    $expected2_1->totalIof = 45.25;
    $expected2_1->contractAmount = 7845.25;
    $expected2_1->contractAmountWithoutTac = 0.0;
    $expected2_1->tacAmount = 0.0;
    $expected2_1->iofPercentage = 0.000082;
    $expected2_1->overallIof = 0.0038;
    $expected2_1->preDisbursementAmount = 7800.0;
    $expected2_1->paidTotalIof = 45.25;
    $expected2_1->paidContractAmount = 7845.25;
    $expected2_1->invoices = [$expectedInvoice2_1];



    $expected2_2 = new Response();
    $expected2_2->installment = 2;
    $expected2_2->dueDate = new DateTimeImmutable('2025-08-04');
    $expected2_2->disbursementDate = new DateTimeImmutable('2025-06-09');
    $expected2_2->accumulatedDays = 56;
    $expected2_2->daysIndex = 0.957779256710963;
    $expected2_2->accumulatedDaysIndex = 1.9391512226071321;
    $expected2_2->interestRate = 0.0235;
    $expected2_2->installmentAmount = 4051.09;
    $expected2_2->installmentAmountWithoutTac = 0.0;
    $expected2_2->totalAmount = 8102.18;
    $expected2_2->debitService = 246.50000000000028;
    $expected2_2->customerDebitServiceAmount = 246.50000000000028;
    $expected2_2->customerAmount = 4051.09;
    $expected2_2->calculationBasisForEffectiveInterestRate = 4023.25;
    $expected2_2->merchantDebitServiceAmount = 0.0;
    $expected2_2->merchantTotalAmount = 390.0;
    $expected2_2->settledToMerchant = 7410.0;
    $expected2_2->mdrAmount = 390.0;
    $expected2_2->effectiveInterestRate = 0.0238;
    $expected2_2->totalEffectiveCost = 0.0294;
    $expected2_2->eirYearly = 0.326624;
    $expected2_2->tecYearly = 0.416087;
    $expected2_2->eirMonthly = 0.0238;
    $expected2_2->tecMonthly = 0.0294;
    $expected2_2->totalIof = 55.68;
    $expected2_2->contractAmount = 7855.68;
    $expected2_2->contractAmountWithoutTac = 0.0;
    $expected2_2->tacAmount = 0.0;
    $expected2_2->iofPercentage = 0.000082;
    $expected2_2->overallIof = 0.0038;
    $expected2_2->preDisbursementAmount = 7800.0;
    $expected2_2->paidTotalIof = 55.68;
    $expected2_2->paidContractAmount = 7855.68;
    $expected2_2->invoices = [$expectedInvoice2_1, $expectedInvoice2_2];



    $expected2_3 = new Response();
    $expected2_3->installment = 3;
    $expected2_3->dueDate = new DateTimeImmutable('2025-09-03');
    $expected2_3->disbursementDate = new DateTimeImmutable('2025-06-09');
    $expected2_3->accumulatedDays = 86;
    $expected2_3->daysIndex = 0.93475372892694;
    $expected2_3->accumulatedDaysIndex = 2.873904951534072;
    $expected2_3->interestRate = 0.0235;
    $expected2_3->installmentAmount = 2736.97;
    $expected2_3->installmentAmountWithoutTac = 0.0;
    $expected2_3->totalAmount = 8210.91;
    $expected2_3->debitService = 345.11999999999983;
    $expected2_3->customerDebitServiceAmount = 345.11999999999983;
    $expected2_3->customerAmount = 2736.97;
    $expected2_3->calculationBasisForEffectiveInterestRate = 2715.04;
    $expected2_3->merchantDebitServiceAmount = 0.0;
    $expected2_3->merchantTotalAmount = 390.0;
    $expected2_3->settledToMerchant = 7410.0;
    $expected2_3->mdrAmount = 390.0;
    $expected2_3->effectiveInterestRate = 0.024;
    $expected2_3->totalEffectiveCost = 0.0288;
    $expected2_3->eirYearly = 0.329156;
    $expected2_3->tecYearly = 0.405648;
    $expected2_3->eirMonthly = 0.024;
    $expected2_3->tecMonthly = 0.0288;
    $expected2_3->totalIof = 65.79;
    $expected2_3->contractAmount = 7865.79;
    $expected2_3->contractAmountWithoutTac = 0.0;
    $expected2_3->tacAmount = 0.0;
    $expected2_3->iofPercentage = 0.000082;
    $expected2_3->overallIof = 0.0038;
    $expected2_3->preDisbursementAmount = 7800.0;
    $expected2_3->paidTotalIof = 65.79;
    $expected2_3->paidContractAmount = 7865.79;
    $expected2_3->invoices = [$expectedInvoice2_1, $expectedInvoice2_2, $expectedInvoice2_3];



    $expected2_4 = new Response();
    $expected2_4->installment = 4;
    $expected2_4->dueDate = new DateTimeImmutable('2025-10-03');
    $expected2_4->disbursementDate = new DateTimeImmutable('2025-06-09');
    $expected2_4->accumulatedDays = 116;
    $expected2_4->daysIndex = 0.912281747198563;
    $expected2_4->accumulatedDaysIndex = 3.786186698732635;
    $expected2_4->interestRate = 0.0235;
    $expected2_4->installmentAmount = 2080.16;
    $expected2_4->installmentAmountWithoutTac = 0.0;
    $expected2_4->totalAmount = 8320.64;
    $expected2_4->debitService = 444.74999999999943;
    $expected2_4->customerDebitServiceAmount = 444.74999999999943;
    $expected2_4->customerAmount = 2080.16;
    $expected2_4->calculationBasisForEffectiveInterestRate = 2061.1875;
    $expected2_4->merchantDebitServiceAmount = 0.0;
    $expected2_4->merchantTotalAmount = 390.0;
    $expected2_4->settledToMerchant = 7410.0;
    $expected2_4->mdrAmount = 390.0;
    $expected2_4->effectiveInterestRate = 0.0241;
    $expected2_4->totalEffectiveCost = 0.0285;
    $expected2_4->eirYearly = 0.331464;
    $expected2_4->tecYearly = 0.400907;
    $expected2_4->eirMonthly = 0.0241;
    $expected2_4->tecMonthly = 0.0285;
    $expected2_4->totalIof = 75.89;
    $expected2_4->contractAmount = 7875.89;
    $expected2_4->contractAmountWithoutTac = 0.0;
    $expected2_4->tacAmount = 0.0;
    $expected2_4->iofPercentage = 0.000082;
    $expected2_4->overallIof = 0.0038;
    $expected2_4->preDisbursementAmount = 7799.98;
    $expected2_4->paidTotalIof = 75.87;
    $expected2_4->paidContractAmount = 7875.87;
    $expected2_4->invoices = [$expectedInvoice2_1, $expectedInvoice2_2, $expectedInvoice2_3, $expectedInvoice2_4];


    $expected3_1 = new Response();
    $expected3_1->installment = 1;
    $expected3_1->dueDate = new DateTimeImmutable('2025-08-04');
    $expected3_1->disbursementDate = new DateTimeImmutable('2025-07-09');
    $expected3_1->accumulatedDays = 26;
    $expected3_1->daysIndex = 0.980287069256833;
    $expected3_1->accumulatedDaysIndex = 0.980287069256833;
    $expected3_1->interestRate = 0.0235;
    $expected3_1->installmentAmount = 8004.34;
    $expected3_1->installmentAmountWithoutTac = 0.0;
    $expected3_1->totalAmount = 8004.34;
    $expected3_1->debitService = 157.79000000000013;
    $expected3_1->customerDebitServiceAmount = 157.79000000000013;
    $expected3_1->customerAmount = 8004.34;
    $expected3_1->calculationBasisForEffectiveInterestRate = 7957.79;
    $expected3_1->merchantDebitServiceAmount = 0.0;
    $expected3_1->merchantTotalAmount = 390.0;
    $expected3_1->settledToMerchant = 7410.0;
    $expected3_1->mdrAmount = 390.0;
    $expected3_1->effectiveInterestRate = 0.0236;
    $expected3_1->totalEffectiveCost = 0.0307;
    $expected3_1->eirYearly = 0.322466;
    $expected3_1->tecYearly = 0.437689;
    $expected3_1->eirMonthly = 0.0236;
    $expected3_1->tecMonthly = 0.0307;
    $expected3_1->totalIof = 46.55;
    $expected3_1->contractAmount = 7846.55;
    $expected3_1->contractAmountWithoutTac = 0.0;
    $expected3_1->tacAmount = 0.0;
    $expected3_1->iofPercentage = 0.000082;
    $expected3_1->overallIof = 0.0038;
    $expected3_1->preDisbursementAmount = 7800.0;
    $expected3_1->paidTotalIof = 46.55;
    $expected3_1->paidContractAmount = 7846.55;
    $expected3_1->invoices = [$expectedInvoice3_1];



    $expected3_2 = new Response();
    $expected3_2->installment = 2;
    $expected3_2->dueDate = new DateTimeImmutable('2025-09-03');
    $expected3_2->disbursementDate = new DateTimeImmutable('2025-07-09');
    $expected3_2->accumulatedDays = 56;
    $expected3_2->daysIndex = 0.956720441569568;
    $expected3_2->accumulatedDaysIndex = 1.937007510826401;
    $expected3_2->interestRate = 0.0235;
    $expected3_2->installmentAmount = 4055.92;
    $expected3_2->installmentAmountWithoutTac = 0.0;
    $expected3_2->totalAmount = 8111.84;
    $expected3_2->debitService = 255.50000000000014;
    $expected3_2->customerDebitServiceAmount = 255.50000000000014;
    $expected3_2->customerAmount = 4055.92;
    $expected3_2->calculationBasisForEffectiveInterestRate = 4027.75;
    $expected3_2->merchantDebitServiceAmount = 0.0;
    $expected3_2->merchantTotalAmount = 390.0;
    $expected3_2->settledToMerchant = 7410.0;
    $expected3_2->mdrAmount = 390.0;
    $expected3_2->effectiveInterestRate = 0.0241;
    $expected3_2->totalEffectiveCost = 0.0296;
    $expected3_2->eirYearly = 0.330449;
    $expected3_2->tecYearly = 0.418932;
    $expected3_2->eirMonthly = 0.0241;
    $expected3_2->tecMonthly = 0.0296;
    $expected3_2->totalIof = 56.34;
    $expected3_2->contractAmount = 7856.34;
    $expected3_2->contractAmountWithoutTac = 0.0;
    $expected3_2->tacAmount = 0.0;
    $expected3_2->iofPercentage = 0.000082;
    $expected3_2->overallIof = 0.0038;
    $expected3_2->preDisbursementAmount = 7800.01;
    $expected3_2->paidTotalIof = 56.35;
    $expected3_2->paidContractAmount = 7856.35;
    $expected3_2->invoices = [$expectedInvoice3_1, $expectedInvoice3_2];


    $expected3_3 = new Response();
    $expected3_3->installment = 3;
    $expected3_3->dueDate = new DateTimeImmutable('2025-10-03');
    $expected3_3->disbursementDate = new DateTimeImmutable('2025-07-09');
    $expected3_3->accumulatedDays = 86;
    $expected3_3->daysIndex = 0.933720368270266;
    $expected3_3->accumulatedDaysIndex = 2.870727879096667;
    $expected3_3->interestRate = 0.0235;
    $expected3_3->installmentAmount = 2740.16;
    $expected3_3->installmentAmountWithoutTac = 0.0;
    $expected3_3->totalAmount = 8220.48;
    $expected3_3->debitService = 354.23999999999955;
    $expected3_3->customerDebitServiceAmount = 354.23999999999955;
    $expected3_3->customerAmount = 2740.16;
    $expected3_3->calculationBasisForEffectiveInterestRate = 2718.08;
    $expected3_3->merchantDebitServiceAmount = 0.0;
    $expected3_3->merchantTotalAmount = 390.0;
    $expected3_3->settledToMerchant = 7410.0;
    $expected3_3->mdrAmount = 390.0;
    $expected3_3->effectiveInterestRate = 0.0243;
    $expected3_3->totalEffectiveCost = 0.0291;
    $expected3_3->eirYearly = 0.334168;
    $expected3_3->tecYearly = 0.410516;
    $expected3_3->eirMonthly = 0.0243;
    $expected3_3->tecMonthly = 0.0291;
    $expected3_3->totalIof = 66.24;
    $expected3_3->contractAmount = 7866.24;
    $expected3_3->contractAmountWithoutTac = 0.0;
    $expected3_3->tacAmount = 0.0;
    $expected3_3->iofPercentage = 0.000082;
    $expected3_3->overallIof = 0.0038;
    $expected3_3->preDisbursementAmount = 7800.01;
    $expected3_3->paidTotalIof = 66.25;
    $expected3_3->paidContractAmount = 7866.25;
    $expected3_3->invoices = [$expectedInvoice3_1, $expectedInvoice3_2, $expectedInvoice3_3];



    $expected3_4 = new Response();
    $expected3_4->installment = 4;
    $expected3_4->dueDate = new DateTimeImmutable('2025-11-03');
    $expected3_4->disbursementDate = new DateTimeImmutable('2025-07-09');
    $expected3_4->accumulatedDays = 117;
    $expected3_4->daysIndex = 0.912281747198563;
    $expected3_4->accumulatedDaysIndex = 3.78300962629523;
    $expected3_4->interestRate = 0.0235;
    $expected3_4->installmentAmount = 2082.05;
    $expected3_4->installmentAmountWithoutTac = 0.0;
    $expected3_4->totalAmount = 8328.2;
    $expected3_4->debitService = 451.7800000000007;
    $expected3_4->customerDebitServiceAmount = 451.7800000000007;
    $expected3_4->customerAmount = 2082.05;
    $expected3_4->calculationBasisForEffectiveInterestRate = 2062.945;
    $expected3_4->merchantDebitServiceAmount = 0.0;
    $expected3_4->merchantTotalAmount = 390.0;
    $expected3_4->settledToMerchant = 7410.0;
    $expected3_4->mdrAmount = 390.0;
    $expected3_4->effectiveInterestRate = 0.0243;
    $expected3_4->totalEffectiveCost = 0.0286;
    $expected3_4->eirYearly = 0.33315;
    $expected3_4->tecYearly = 0.402404;
    $expected3_4->eirMonthly = 0.0243;
    $expected3_4->tecMonthly = 0.0286;
    $expected3_4->totalIof = 76.42;
    $expected3_4->contractAmount = 7876.42;
    $expected3_4->contractAmountWithoutTac = 0.0;
    $expected3_4->tacAmount = 0.0;
    $expected3_4->iofPercentage = 0.000082;
    $expected3_4->overallIof = 0.0038;
    $expected3_4->preDisbursementAmount = 7800.0;
    $expected3_4->paidTotalIof = 76.42;
    $expected3_4->paidContractAmount = 7876.42;
    $expected3_4->invoices = [$expectedInvoice3_1, $expectedInvoice3_2, $expectedInvoice3_3, $expectedInvoice3_4];



    $expected4_1 = new Response();
    $expected4_1->installment = 1;
    $expected4_1->dueDate = new DateTimeImmutable('2025-09-03');
    $expected4_1->disbursementDate = new DateTimeImmutable('2025-08-11');
    $expected4_1->accumulatedDays = 23;
    $expected4_1->daysIndex = 0.981371965896169;
    $expected4_1->accumulatedDaysIndex = 0.981371965896169;
    $expected4_1->interestRate = 0.0235;
    $expected4_1->installmentAmount = 7993.5;
    $expected4_1->installmentAmountWithoutTac = 0.0;
    $expected4_1->totalAmount = 7993.5;
    $expected4_1->debitService = 148.9;
    $expected4_1->customerDebitServiceAmount = 148.9;
    $expected4_1->customerAmount = 7993.5;
    $expected4_1->calculationBasisForEffectiveInterestRate = 7948.9;
    $expected4_1->merchantDebitServiceAmount = 0.0;
    $expected4_1->merchantTotalAmount = 390.0;
    $expected4_1->settledToMerchant = 7410.0;
    $expected4_1->mdrAmount = 390.0;
    $expected4_1->effectiveInterestRate = 0.0252;
    $expected4_1->totalEffectiveCost = 0.0329;
    $expected4_1->eirYearly = 0.347719;
    $expected4_1->tecYearly = 0.475332;
    $expected4_1->eirMonthly = 0.0252;
    $expected4_1->tecMonthly = 0.0329;
    $expected4_1->totalIof = 44.6;
    $expected4_1->contractAmount = 7844.6;
    $expected4_1->contractAmountWithoutTac = 0.0;
    $expected4_1->tacAmount = 0.0;
    $expected4_1->iofPercentage = 0.000082;
    $expected4_1->overallIof = 0.0038;
    $expected4_1->preDisbursementAmount = 7800.0;
    $expected4_1->paidTotalIof = 44.6;
    $expected4_1->paidContractAmount = 7844.6;
    $expected4_1->invoices = [$expectedInvoice4_1];


    $expected4_2 = new Response();
    $expected4_2->installment = 2;
    $expected4_2->dueDate = new DateTimeImmutable('2025-10-03');
    $expected4_2->disbursementDate = new DateTimeImmutable('2025-08-11');
    $expected4_2->accumulatedDays = 53;
    $expected4_2->daysIndex = 0.957779256710963;
    $expected4_2->accumulatedDaysIndex = 1.9391512226071321;
    $expected4_2->interestRate = 0.0235;
    $expected4_2->installmentAmount = 4050.43;
    $expected4_2->installmentAmountWithoutTac = 0.0;
    $expected4_2->totalAmount = 8100.86;
    $expected4_2->debitService = 246.4699999999997;
    $expected4_2->customerDebitServiceAmount = 246.4699999999997;
    $expected4_2->customerAmount = 4050.43;
    $expected4_2->calculationBasisForEffectiveInterestRate = 4023.2349999999997;
    $expected4_2->merchantDebitServiceAmount = 0.0;
    $expected4_2->merchantTotalAmount = 390.0;
    $expected4_2->settledToMerchant = 7410.0;
    $expected4_2->mdrAmount = 390.0;
    $expected4_2->effectiveInterestRate = 0.0251;
    $expected4_2->totalEffectiveCost = 0.0308;
    $expected4_2->eirYearly = 0.34648;
    $expected4_2->tecYearly = 0.439943;
    $expected4_2->eirMonthly = 0.0251;
    $expected4_2->tecMonthly = 0.0308;
    $expected4_2->totalIof = 54.39;
    $expected4_2->contractAmount = 7854.39;
    $expected4_2->contractAmountWithoutTac = 0.0;
    $expected4_2->tacAmount = 0.0;
    $expected4_2->iofPercentage = 0.000082;
    $expected4_2->overallIof = 0.0038;
    $expected4_2->preDisbursementAmount = 7800.01;
    $expected4_2->paidTotalIof = 54.4;
    $expected4_2->paidContractAmount = 7854.4;
    $expected4_2->invoices = [$expectedInvoice4_1, $expectedInvoice4_2];



    $expected4_3 = new Response();
    $expected4_3->installment = 3;
    $expected4_3->dueDate = new DateTimeImmutable('2025-11-03');
    $expected4_3->disbursementDate = new DateTimeImmutable('2025-08-11');
    $expected4_3->accumulatedDays = 84;
    $expected4_3->daysIndex = 0.935788233217493;
    $expected4_3->accumulatedDaysIndex = 2.874939455824625;
    $expected4_3->interestRate = 0.0235;
    $expected4_3->installmentAmount = 2735.54;
    $expected4_3->installmentAmountWithoutTac = 0.0;
    $expected4_3->totalAmount = 8206.62;
    $expected4_3->debitService = 342.1200000000008;
    $expected4_3->customerDebitServiceAmount = 342.1200000000008;
    $expected4_3->customerAmount = 2735.54;
    $expected4_3->calculationBasisForEffectiveInterestRate = 2714.0400000000004;
    $expected4_3->merchantDebitServiceAmount = 0.0;
    $expected4_3->merchantTotalAmount = 390.0;
    $expected4_3->settledToMerchant = 7410.0;
    $expected4_3->mdrAmount = 390.0;
    $expected4_3->effectiveInterestRate = 0.0247;
    $expected4_3->totalEffectiveCost = 0.0296;
    $expected4_3->eirYearly = 0.340141;
    $expected4_3->tecYearly = 0.418684;
    $expected4_3->eirMonthly = 0.0247;
    $expected4_3->tecMonthly = 0.0296;
    $expected4_3->totalIof = 64.5;
    $expected4_3->contractAmount = 7864.5;
    $expected4_3->contractAmountWithoutTac = 0.0;
    $expected4_3->tacAmount = 0.0;
    $expected4_3->iofPercentage = 0.000082;
    $expected4_3->overallIof = 0.0038;
    $expected4_3->preDisbursementAmount = 7800.01;
    $expected4_3->paidTotalIof = 64.51;
    $expected4_3->paidContractAmount = 7864.51;
    $expected4_3->invoices = [$expectedInvoice4_1, $expectedInvoice4_2, $expectedInvoice4_3];


    $expected4_4 = new Response();
    $expected4_4->installment = 4;
    $expected4_4->dueDate = new DateTimeImmutable('2025-12-03');
    $expected4_4->disbursementDate = new DateTimeImmutable('2025-08-11');
    $expected4_4->accumulatedDays = 114;
    $expected4_4->daysIndex = 0.914302133077605;
    $expected4_4->accumulatedDaysIndex = 3.78924158890223;
    $expected4_4->interestRate = 0.0235;
    $expected4_4->installmentAmount = 2078.15;
    $expected4_4->installmentAmountWithoutTac = 0.0;
    $expected4_4->totalAmount = 8312.6;
    $expected4_4->debitService = 437.99000000000035;
    $expected4_4->customerDebitServiceAmount = 437.99000000000035;
    $expected4_4->customerAmount = 2078.15;
    $expected4_4->calculationBasisForEffectiveInterestRate = 2059.4975;
    $expected4_4->merchantDebitServiceAmount = 0.0;
    $expected4_4->merchantTotalAmount = 390.0;
    $expected4_4->settledToMerchant = 7410.0;
    $expected4_4->mdrAmount = 390.0;
    $expected4_4->effectiveInterestRate = 0.0245;
    $expected4_4->totalEffectiveCost = 0.0289;
    $expected4_4->eirYearly = 0.336923;
    $expected4_4->tecYearly = 0.407548;
    $expected4_4->eirMonthly = 0.0245;
    $expected4_4->tecMonthly = 0.0289;
    $expected4_4->totalIof = 74.61;
    $expected4_4->contractAmount = 7874.61;
    $expected4_4->contractAmountWithoutTac = 0;
    $expected4_4->tacAmount = 0.0;
    $expected4_4->iofPercentage = 0.000082;
    $expected4_4->overallIof = 0.0038;
    $expected4_4->preDisbursementAmount = 7800.0;
    $expected4_4->paidTotalIof = 74.61;
    $expected4_4->paidContractAmount = 7874.61;
    $expected4_4->invoices = [$expectedInvoice4_1, $expectedInvoice4_2, $expectedInvoice4_3, $expectedInvoice4_4];

    $expectedDownPayment1 = new DownPaymentResponse();
    $expectedDownPayment1->installmentAmount = 1000;
    $expectedDownPayment1->totalAmount = 1000;
    $expectedDownPayment1->installmentQuantity = 1;
    $expectedDownPayment1->firstPaymentDate = new DateTimeImmutable('2025-05-03');
    $expectedDownPayment1->plans = [
      $expected1_1,
      $expected1_2,
      $expected1_3,
      $expected1_4
    ];



    $expectedDownPayment2 = new DownPaymentResponse();
    $expectedDownPayment2->installmentAmount = 500;
    $expectedDownPayment2->totalAmount = 1000;
    $expectedDownPayment2->installmentQuantity = 2;
    $expectedDownPayment2->firstPaymentDate = new DateTimeImmutable('2025-05-03');
    $expectedDownPayment2->plans = [
      $expected2_1,
      $expected2_2,
      $expected2_3,
      $expected2_4
    ];


    $expectedDownPayment3 = new DownPaymentResponse();
    $expectedDownPayment3->installmentAmount = 333.3333333333333;
    $expectedDownPayment3->totalAmount = 1000;
    $expectedDownPayment3->installmentQuantity = 3;
    $expectedDownPayment3->firstPaymentDate = new DateTimeImmutable('2025-05-03');
    $expectedDownPayment3->plans = [
      $expected3_1,
      $expected3_2,
      $expected3_3,
      $expected3_4
    ];

    $expectedDownPayment4 = new DownPaymentResponse();
    $expectedDownPayment4->installmentAmount = 250.0;
    $expectedDownPayment4->totalAmount = 1000.0;
    $expectedDownPayment4->installmentQuantity = 4;
    $expectedDownPayment4->firstPaymentDate = new DateTimeImmutable('2025-05-03');
    $expectedDownPayment4->plans = [
      $expected4_1,
      $expected4_2,
      $expected4_3,
      $expected4_4
    ];


    $params = new Params(
      7800.0, // requested_amount
      new DateTimeImmutable('2025-05-03 05:00:00'), // first_payment_date
      new DateTimeImmutable('2025-04-03 05:00:00'), // disbursement_date
      4,     // installments
      0,      // debit_service_percentage
      0.05,   // mdr
      0.0,    // tac_percentage
      0.0038, // iof_overall
      0.000082, // iof_percentage
      0.0235, // interest_rate
      100.0,   // min_installment_amount
      1000000, // max_total_amount
      true    // disbursement_only_on_business_days
    );

    $downPaymentParams = new DownPaymentParams(
      1000.0, // requested_amount
      100.0,  // min_installment_amount
      new DateTimeImmutable('2025-05-03'), // first_payment_date
      4,       // installments
      $params
    );
    $resp = PaymentPlan::calculateDownPayment(
      $downPaymentParams
    );

    $this->assertIsArray($resp);
    $this->assertNotEmpty($resp);
    $this->assertEquals(4, count($resp));
    $this->assertIsObject($resp[0]);
    $this->assertEquals($expectedDownPayment1, $resp[0]);
    $this->assertEquals($expectedDownPayment2, $resp[1]);
    $this->assertEquals($expectedDownPayment3, $resp[2]);
    $this->assertEquals($expectedDownPayment4, $resp[3]);
  }


  public function testDisbursementDateRange()
  {
    $range = PaymentPlan::disbursementDateRange(
      new DateTimeImmutable('2025-04-03'),
      5
    );

    $expectedStart = new DateTimeImmutable('2025-04-03 10:00:00');
    $expectedEnd = new DateTimeImmutable('2025-04-09 10:00:00');

    $this->assertIsArray($range);
    $this->assertNotEmpty($range);
    $this->assertCount(2, $range);
    $this->assertInstanceOf(DateTimeImmutable::class, $range[0]);
    $this->assertInstanceOf(DateTimeImmutable::class, $range[1]);
    $this->assertEquals($expectedStart, $range[0]);
    $this->assertEquals($expectedEnd, $range[1]);
  }

  public function testNextDisbursementDate()
  {
    $nextDate = PaymentPlan::nextDisbursementDate(
      new DateTimeImmutable('2025-04-03')
    );

    $expectedDate = new DateTimeImmutable('2025-04-03 10:00:00');

    $this->assertInstanceOf(DateTimeImmutable::class, $nextDate);
    $this->assertEquals($expectedDate, $nextDate);
  }

  public function testGetNonBusinessDaysBetween()
  {
    $nonBusinessDays = PaymentPlan::getNonBusinessDaysBetween(
      new DateTimeImmutable('2025-04-01'),
      new DateTimeImmutable('2025-04-30')
    );



    $expectedNonBusinessDays = [
      new DateTimeImmutable('2025-04-05 10:00:00'),
      new DateTimeImmutable('2025-04-06 10:00:00'),
      new DateTimeImmutable('2025-04-12 10:00:00'),
      new DateTimeImmutable('2025-04-13 10:00:00'),
      new DateTimeImmutable('2025-04-18 10:00:00'),
      new DateTimeImmutable('2025-04-19 10:00:00'),
      new DateTimeImmutable('2025-04-20 10:00:00'),
      new DateTimeImmutable('2025-04-21 10:00:00'),
      new DateTimeImmutable('2025-04-26 10:00:00'),
      new DateTimeImmutable('2025-04-27 10:00:00')
    ];

    $this->assertIsArray($nonBusinessDays);
    $this->assertNotEmpty($nonBusinessDays);
    $this->assertCount(10, $nonBusinessDays);
    foreach ($nonBusinessDays as $day) {
      $this->assertInstanceOf(DateTimeImmutable::class, $day);
    }
    $this->assertEquals($expectedNonBusinessDays, $nonBusinessDays);
  }
}
