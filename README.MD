# Workday Checker

Workday Checker is a Node.js package that allows you to check whether a given date is a workday or not, considering weekends and user-defined holidays.

## Installation

To install Workday Checker, you can use npm:

```bash
npm install workday-checker
```

## Usage

```javascript
const workdayChecker = require('workday-checker');

// Set locale and update holidays
workdayChecker.update('en-US', ['2024-01-01', '2024-12-25']); // Update locale and holidays

// Check if a date is a workday
const date = new Date('2024-04-15');
const isWorkday = workdayChecker.isWorkday(date);
console.log(isWorkday); // Output: true

// Get the next workday
const nextWorkday = workdayChecker.getNextWorkday(date);
console.log(nextWorkday); // Output: 2024-04-16

// Check if a date is a holiday
const isHoliday = workdayChecker.isHoliday(date);
console.log(isHoliday); // Output: false

// Add a new holiday
workdayChecker.addHoliday('2024-05-01'); // Add May 1st as a holiday

// Remove a holiday
workdayChecker.removeHoliday('2024-12-25'); // Remove December 25th as a holiday

// Get workdays within a date range
const startDate = new Date('2024-04-01');
const endDate = new Date('2024-04-30');
const workdaysInRange = workdayChecker.getWorkdaysInRange(startDate, endDate);
console.log(workdaysInRange); // Output: Array of workdays between April 1st and April 30th

// Get the name of a holiday
const holidayName = workdayChecker.getHolidayName(date);
console.log(holidayName); // Output: Holiday name if the date is a holiday, otherwise undefined
```

## API

### `update(newLocale: string, newHolidays: string[]): void`

Updates the locale and the list of holidays.

- `newLocale`: The new locale to set.
- `newHolidays`: An array of new holidays to add.

### `isWorkday(date: Date): boolean`

Checks if a given date is a workday.

- `date`: The date to check.

### `getNextWorkday(date: Date): Date`

Returns the next workday after the given date.

- `date`: The date for which to find the next workday.

### `isHoliday(date: Date): boolean`

Checks if a given date is a holiday.

- `date`: The date to check.

### `addHoliday(date: string): void`

Adds a new holiday to the list of holidays.

- `date`: The date of the holiday to add (in format 'YYYY-MM-DD').

### `removeHoliday(date: string): void`

Removes a holiday from the list of holidays.

- `date`: The date of the holiday to remove (in format 'YYYY-MM-DD').

### `getWorkdaysInRange(startDate: Date, endDate: Date): Date[]`

Returns an array of workdays within the specified date range.

- `startDate`: The start date of the range.
- `endDate`: The end date of the range.

### `getHolidayName(date: Date): string | undefined`

Returns the name of the holiday for the given date.

- `date`: The date for which to get the holiday name.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
