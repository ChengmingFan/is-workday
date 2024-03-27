const { DateTime } = require('luxon');

let locale = 'en-US'; // Default locale
let holidays = []; // Default holidays list

function isWorkingday(date, locale) {
    const dt = DateTime.fromJSDate(date, { locale });

    // Check if it's a weekend
    if (dt.weekday > 5) {
        return false;
    }

    // Check if it's a holiday
    const formattedDate = dt.toISODate();
    return !holidays.includes(formattedDate);

}

function update(newLocale, newHolidays) {
    locale = newLocale;
    holidays = [...holidays, ...newHolidays]; // Add new holidays to the existing list
}

module.exports = {
    isWorkingday,
    update
};

