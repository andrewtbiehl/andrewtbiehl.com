---
---

/**
The special dates program (specifically implementation 2B), ported to JavaScript.

This program is used in the essay "Turn that code upside down :)". For more
information, please refer to that essay.
*/

(function () {

"use strict";

const SIMPLE_DATE_FORMAT_OPTIONS = {
  year: "2-digit",
  month: "numeric",
  day: "numeric",
};
const ORDINARY_MSG_END = "just an ordinary date.";
const SPECIAL_MSG_END = "what a special date!";

function main() {
  // Imperative shell
  const today = new Date();
  // Functional core
  const digits_ = digits(today);
  const consistsOfSingleDigit = consistsOfSingleVal(digits_);
  const isSequential_ = isSequential(digits_);
  const isSpecial_ = isSpecial(consistsOfSingleDigit, isSequential_);
  const msg = createMsg(today, isSpecial_);
  // Imperative shell
  document.querySelector("#special-dates-example-id > code > span.go")
    .textContent = msg;
}

/** Returns the integers comprising the US date format of the given date. */
function digits(date_) {
  const digitStr = date_.simpleUsFormat().replaceAll("/", "").split("");
  return digitStr.map(Number);
}

/** Checks whether the collection consists of a single "repeated" value. */
function consistsOfSingleVal(xs) {
  return new Set(xs).size <= 1;
}

/** Checks whether the sequence is increasing or decreasing by exactly 1. */
function isSequential(nums) {
  const pairs = nums.pairwise();
  const differences = pairs.map(difference);
  const differenceVals = new Set(differences);
  return (
    differenceVals.size == 1 &&
    (differenceVals.has(1) || differenceVals.has(-1))
  );
}

/** Returns the difference between the second and first item in the pair. */
function difference(pair) {
  return pair[1] - pair[0];
}

/** Computes whether the given properties amount to being "special". */
function isSpecial(consistsOfSingleDigit, isSequential_) {
  return consistsOfSingleDigit || isSequential_;
}

/** Returns a message describing whether the given date is special. */
function createMsg(date_, isSpecial_) {
  const msgEnd = isSpecial_ ? SPECIAL_MSG_END : ORDINARY_MSG_END;
  return `Today is ${date_.simpleUsFormat()}, ${msgEnd}`;
}

/** Returns the simple US format of the given date. */
Date.prototype.simpleUsFormat = function () {
  return this.toLocaleDateString("en-US", SIMPLE_DATE_FORMAT_OPTIONS);
};

/** Return an array of overlapping pairs from the input array. */
Array.prototype.pairwise = function () {
  return this.map(function (current, currentIndex, this_) {
    const nextIndex = currentIndex + 1;
    const next = this_[nextIndex];
    return [current, next];
  }).slice(0, -1);
};

main();

})();
