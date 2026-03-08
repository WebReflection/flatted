const { parse, stringify } = require('../cjs');

const AMOUNT = 5500;

// Build a chain of 5500 linked objects
let chain = ["leaf"];
for (let i = 0; i < AMOUNT; i++) {
  chain = [chain];
}

const str = stringify(chain);
console.log('stringify', '✅');
// console.log(str);

const parsed = parse(str);

let current = parsed;
for (let i = 0; i < AMOUNT; i++) {
  current = current[0];
}
console.log(current);
console.log('parse', '✅');
