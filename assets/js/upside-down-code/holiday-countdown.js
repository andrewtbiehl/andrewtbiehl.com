---
---

/**
The holiday countdown program (specifically implementation 1B), ported to JavaScript.

This program is used in the essay "Turn that code upside down :)". For more
information, please refer to that essay.
*/

(function () {

"use strict";

class Holiday {
  constructor(name, date) {
    this.name = name;
    this.date = date;
  }
}

const MILLIS_PER_SECOND = 1000;
const SECONDS_PER_MINUTE = 60;
const MINUTES_PER_HOUR = 60;
const HOURS_PER_DAY = 24;
const MILLIS_PER_DAY =
  MILLIS_PER_SECOND *
  SECONDS_PER_MINUTE *
  MINUTES_PER_HOUR *
  HOURS_PER_DAY;

async function main() {
  const today = new Date();
  const urls = buildNearbyHolidayUrls(today);
  const holidayPages = await Promise.all(urls.map(fetchJson));
  const nearbyHolidays = toHolidays(holidayPages);
  const nextHoliday = findNextHoliday(today, nearbyHolidays);
  const numDays = daysAfter(today, nextHoliday);
  const holidayName = nextHoliday.name;
  const msg = `Only ${numDays} more day(s) until ${holidayName}!`;
  document.querySelector("#holiday-countdown-example-id > code > span.go")
    .textContent = msg;
}

/** Fetches JSON from a web API url. */
async function fetchJson(url) {
  const response = await fetch(url);
  const json = await response.json();
  return json;
}

/** Returns urls for fetching US holidays near the given date. */
function buildNearbyHolidayUrls(date_) {
  const currentYear = date_.getFullYear();
  const nextYear = currentYear + 1;
  const years = [currentYear, nextYear];
  return years.map((year) => `https://date.nager.at/api/v1/Get/US/${year}`);
}

/** Maps multiple raw holiday pages into a collection of holidays. */
function toHolidays(rawHolidayPages) {
  const rawHolidays = rawHolidayPages.flat();
  return rawHolidays.map(toHoliday);
}

/** Maps a dictionary of raw holiday information into a Holiday. */
function toHoliday(rawHoliday) {
  return new Holiday(rawHoliday.name, new Date(rawHoliday.date));
}

/** Finds the next holiday in the given collection after the given date. */
function findNextHoliday(date_, nearbyHolidays) {
  const isUpcoming = (holiday) => isAfter(date_, holiday);
  const upcomingHolidays = nearbyHolidays.filter(isUpcoming);
  return upcomingHolidays.minBy((holiday) => holiday.date);
}

/** Returns the number of days between the given date and holiday. */
function daysAfter(date_, holiday) {
  const millisAfter = holiday.date - date_;
  const daysAfter = Math.ceil(millisAfter / MILLIS_PER_DAY);
  return daysAfter;
}

/** Returns whether the given holiday occurs on or after the given date. */
function isAfter(date_, holiday) {
  return daysAfter(date_, holiday) >= 0;
}

/** Returns the item in the array with the minimum value subject to the given key. */
Array.prototype.minBy = function (key) {
  const head = this[0];
  if (this.length <= 1) {
    return head;
  }
  const tail = this.slice(1);
  const tailMin = tail.minBy(key);
  return key(head) <= key(tailMin) ? head : tailMin;
};

main();

})();
