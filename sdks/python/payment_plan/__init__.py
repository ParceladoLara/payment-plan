from datetime import datetime
from typing import List, Tuple
from ._internal.payment_plan_uniffi import (
    DownPaymentParams,
    DownPaymentResponse,
    Params,
    Response,
    calculate_down_payment_plan,
    calculate_payment_plan,
    next_disbursement_date as _next_disbursement_date,
    disbursement_date_range as _disbursement_date_range,
    get_non_business_days_between as _get_non_business_days_between,
    get_non_business_days_between,
)

def next_disbursement_date(base_date: datetime) -> datetime:
    """
    Calculates the next disbursement date based on the given base date.

    This function assumes disbursement dates on business days only.
    This function also assumes that the disbursement day can't occur on the same day as the system date,
    so in this case +1 day is added no matter what.
    base_dates in the past are allowed, for debugging purposes, but keep the rule of not being the same day in mind.

    Args:
        base_date (datetime): The base date to calculate the next disbursement date.

    Returns:
        datetime: The next disbursement date.
    """
    return _next_disbursement_date(base_date)

def disbursement_date_range(base_date: datetime, days: int) -> Tuple[datetime, datetime]:
    """
    Calculates and returns (start, end) disbursement dates based on the given base date and number of days.

    The start date is the next disbursement date, and the end date is the end of a range that fits the number of business days.
    For example:
        base_date = "2025-04-03"
        days = 5
        start = "2025-04-03" (assuming that the call was not made on "2025-04-03")
        end = "2025-04-09"

    This range fits 5 business days: 2025-04-03, 2025-04-04, 2025-04-07, 2025-04-08, and 2025-04-09.

    This function assumes disbursement dates on business days only.
    This function also assumes that the disbursement day can't occur on the same day as the system date,
    so in this case +1 day is added no matter what.
    base_dates in the past are allowed, for debugging purposes, but keep the rule of not being the same day in mind.

    Args:
        base_date (datetime): The base date to calculate the disbursement date range.
        days (int): The number of business days in the range.

    Returns:
        Tuple[datetime, datetime]: A tuple containing the start and end disbursement dates.
    """
    result = _disbursement_date_range(base_date, days)
    return result[0], result[1]

def get_non_business_days_between(start_date: datetime, end_date: datetime) -> List[datetime]:
    """
    Returns a list of non-business days between the given start and end dates.

    Both start and end dates are inclusive.

    This function assumes disbursement dates on business days only.

    Args:
        start_date (datetime): The start date of the range.
        end_date (datetime): The end date of the range.

    Returns:
        List[datetime]: A list of non-business days within the range.
    """
    result = _get_non_business_days_between(start_date, end_date)
    return result

__all__ = [
    "DownPaymentParams",
    "DownPaymentResponse",
    "Params",
    "Response",
    "calculate_down_payment_plan",
    "calculate_payment_plan",
    "next_disbursement_date",
    "disbursement_date_range",
    "get_non_business_days_between",
]