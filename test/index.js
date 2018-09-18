var Flatted = require('../cjs');

var a = [];
var o = {};

console.assert(Flatted.stringify(a) === '[[]]', 'empty Array');
console.assert(Flatted.stringify(o) === '[{}]', 'empty Object');

a.push(a);
o.o = o;

console.assert(Flatted.stringify(a) === '[["0"]]', 'recursive Array');
console.assert(Flatted.stringify(o) === '[{"o":"0"}]', 'recursive Object');

var b = Flatted.parse(Flatted.stringify(a));
console.assert(Array.isArray(b) && b[0] === b, 'restoring recursive Array');

a.push(1, 'two', true);
o.one = 1;
o.two = 'two';
o.three = true;

console.assert(Flatted.stringify(a) === '[["0",1,"1",true],"two"]', 'values in Array');
console.assert(Flatted.stringify(o) === '[{"o":"0","one":1,"two":"1","three":true},"two"]', 'values in Object');


a.push(o);
o.a = a;

console.assert(Flatted.stringify(a) === '[["0",1,"1",true,"2"],"two",{"o":"2","one":1,"two":"1","three":true,"a":"0"}]', 'object in Array');
console.assert(Flatted.stringify(o) === '[{"o":"0","one":1,"two":"1","three":true,"a":"2"},"two",["2",1,"1",true,"0"]]', 'array in Object');

a.push({test: 'OK'}, [1, 2, 3]);
o.test = {test: 'OK'};
o.array = [1, 2, 3];

console.assert(Flatted.stringify(a) === '[["0",1,"1",true,"2","3","4"],"two",{"o":"2","one":1,"two":"1","three":true,"a":"0","test":"5","array":"6"},{"test":"7"},[1,2,3],{"test":"7"},[1,2,3],"OK"]', 'objects in Array');
console.assert(Flatted.stringify(o) === '[{"o":"0","one":1,"two":"1","three":true,"a":"2","test":"3","array":"4"},"two",["2",1,"1",true,"0","5","6"],{"test":"7"},[1,2,3],{"test":"7"},[1,2,3],"OK"]', 'objects in Object');

a = Flatted.parse(Flatted.stringify(a));
o = Flatted.parse(Flatted.stringify(o));

console.assert(a[0] === a, 'parsed Array');
console.assert(o.o === o, 'parsed Object');

console.assert(
  a[1] === 1 &&
  a[2] === 'two' &&
  a[3] === true &&
  a[4] instanceof Object &&
  JSON.stringify(a[5]) === JSON.stringify({test: 'OK'}) &&
  JSON.stringify(a[6]) === JSON.stringify([1, 2, 3]),
  'array values are all OK'
);

console.assert(a[4] === a[4].o && a === a[4].o.a, 'array recursive values are OK');

console.assert(
  o.one === 1 &&
  o.two === 'two' &&
  o.three === true &&
  Array.isArray(o.a) &&
  JSON.stringify(o.test) === JSON.stringify({test: 'OK'}) &&
  JSON.stringify(o.array) === JSON.stringify([1, 2, 3]),
  'object values are all OK'
);

console.assert(o.a === o.a[0] && o === o.a[4], 'object recursive values are OK');

console.assert(Flatted.parse(Flatted.stringify(1)) === 1, 'numbers can be parsed too');
console.assert(Flatted.parse(Flatted.stringify(false)) === false, 'booleans can be parsed too');
console.assert(Flatted.parse(Flatted.stringify(null)) === null, 'null can be parsed too');
console.assert(Flatted.parse(Flatted.stringify('test')) === 'test', 'strings can be parsed too');

var d = new Date;
console.assert(Flatted.parse(Flatted.stringify(d)) === d.toISOString(), 'dates can be parsed too');

console.assert(Flatted.parse(
  Flatted.stringify(d),
  function (key, value) {
    if (typeof value === 'string' && /^[0-9:.ZT-]+$/.test(value))
      return new Date(value);
    return value;
  }
) instanceof Date, 'dates can be revived too');

console.assert(Flatted.parse(
  Flatted.stringify({
    sub: {
      one23: 123,
      date: d
    }
  }),
  function (key, value) {
    if (key !== '' && typeof value === 'string' && /^[0-9:.ZT-]+$/.test(value))
      return new Date(value);
    return value;
  }
).sub.date instanceof Date, 'dates can be revived too');

