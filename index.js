const { DateTime } = require('luxon');

let locale = 'en-US'; // Default locale
let holidays = []; // Default holidays list

function isWorkday(date, locale) {
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

function getNextWorkday(date, locale) {
    let dt = DateTime.fromJSDate(date, { locale });
    dt = dt.plus({ days: 1 });
    while (!isWorkday(dt.toJSDate(), locale)) {
        dt = dt.plus({ days: 1 });
    }
    return dt.toJSDate();
}

function isHoliday(date, locale) {
    const dt = DateTime.fromJSDate(date, { locale });
    const formattedDate = dt.toISODate();
    return holidays.includes(formattedDate);
}

function addHoliday(date) {
    const formattedDate = DateTime.fromJSDate(date).toISODate();
    if (!holidays.includes(formattedDate)) {
        holidays.push(formattedDate);
    }
}

function removeHoliday(date) {
    const formattedDate = DateTime.fromJSDate(date).toISODate();
    const index = holidays.indexOf(formattedDate);
    if (index !== -1) {
        holidays.splice(index, 1);
    }
}

function getWorkdaysInRange(startDate, endDate, locale) {
    const workdays = [];
    let currentDate = DateTime.fromJSDate(startDate, { locale });
    const end = DateTime.fromJSDate(endDate, { locale });

    while (currentDate <= end) {
        if (isWorkday(currentDate.toJSDate(), locale)) {
            workdays.push(currentDate.toJSDate());
        }
        currentDate = currentDate.plus({ days: 1 });
    }

    return workdays;
}

function getHolidayName(date, locale) {
    const dt = DateTime.fromJSDate(date, { locale });
    const formattedDate = dt.toISODate();
    return holidays[formattedDate];
}

module.exports = {
    isWorkday,
    update,
    getNextWorkday,
    isHoliday,
    addHoliday,
    removeHoliday,
    getWorkdaysInRange,
    getHolidayName
};
