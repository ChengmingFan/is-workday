from datetime import datetime, timedelta

locale = 'en-US'  # Default locale
holidays = []  # Default holidays list


def is_workday(date):
    dt = datetime.strptime(date, '%Y-%m-%d')

    # Check if it's a weekend
    if dt.weekday() > 4:
        return False

    # Check if it's a holiday
    formatted_date = dt.strftime('%Y-%m-%d')
    return formatted_date not in holidays


def update(new_locale, new_holidays):
    global locale, holidays
    locale = new_locale
    holidays.extend(new_holidays)


def get_next_workday(date):
    dt = datetime.strptime(date, '%Y-%m-%d') + timedelta(days=1)
    while not is_workday(dt.strftime('%Y-%m-%d')):
        dt += timedelta(days=1)
    return dt.strftime('%Y-%m-%d')


def is_holiday(date):
    dt = datetime.strptime(date, '%Y-%m-%d')
    formatted_date = dt.strftime('%Y-%m-%d')
    return formatted_date in holidays


def add_holiday(date):
    if date not in holidays:
        holidays.append(date)


def remove_holiday(date):
    if date in holidays:
        holidays.remove(date)


def get_workdays_in_range(start_date, end_date):
    workdays = []
    current_date = datetime.strptime(start_date, '%Y-%m-%d')
    end = datetime.strptime(end_date, '%Y-%m-%d')

    while current_date <= end:
        if is_workday(current_date.strftime('%Y-%m-%d')):
            workdays.append(current_date.strftime('%Y-%m-%d'))
        current_date += timedelta(days=1)

    return workdays


def get_holiday_name(date):
    return holidays.get(date)


# Example usage:
# update('en-US', ['2024-01-01', '2024-12-25'])
# print(is_workday('2024-04-15'))
# print(get_next_workday('2024-04-15'))
# print(is_holiday('2024-04-15'))
# add_holiday('2024-05-01')
# remove_holiday('2024-12-25')
# print(get_workdays_in_range('2024-04-01', '2024-04-30'))
# print(get_holiday_name('2024-04-15'))
