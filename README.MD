# is-workday

A Node.js module to determine whether a given date is a weekday or not, with support for custom locales and holidays.

## Installation

You can install this module using npm:

```bash
npm install is-workday
```

## Usage

```javascript
const { isWeekday, update } = require('is-workday');

// Determine if a specific date is a weekday
const dateToCheck = new Date('2024-03-28');
const locale = 'en-US'; // User's locale
const holidays = ['2024-01-01', '2024-12-25']; // Custom holidays list

if (isWeekday(dateToCheck, locale)) {
    console.log(`${dateToCheck.toDateString()} is a weekday.`);
} else {
    console.log(`${dateToCheck.toDateString()} is not a weekday or is a holiday.`);
}

// Update locale and holidays
const newLocale = 'en-GB'; // New locale
const newHolidays = ['2024-05-01']; // Additional holidays

update(newLocale, newHolidays);
```

## API

### `isWeekday(date, locale)`

Determines whether a given date is a weekday or not.

- `date`: A JavaScript Date object representing the date to be checked.
- `locale`: A string representing the locale to be used for date calculations.

Returns `true` if the date is a weekday, `false` if it's a weekend or a holiday.

### `update(locale, holidays)`

Updates the locale and custom holidays list.

- `locale`: A string repre
