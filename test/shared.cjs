const { parse, stringify } = require(__dirname + '/../cjs');

let arr = ['arr', 1];
let obj = { obj: 2 };

for (let i = 0; i < 64; i++) {
  arr = [arr];
  obj = { obj: obj };
}

const str = stringify([arr, obj]);

require('fs').writeFileSync(__dirname + '/shared.json', str);

[arr, obj] = parse(str);

for (let i = 0; i < 64; i++) {
  arr = arr[0];
  obj = obj.obj;
}

console.assert(
  arr.length === 2 &&
  arr[0] === 'arr' &&
  arr[1] === 1 &&
  obj.obj === 2
);
