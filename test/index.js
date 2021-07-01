var Flatted = require('../cjs');

console.assert(Flatted.stringify([null, null]) === '[[null,null]]', 'multiple null');

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


// borrowed from CircularJSON


(function () {
  var special = "\\x7e"; // \x7e is ~
  //console.log(Flatted.stringify({a:special}));
  //console.log(Flatted.parse(Flatted.stringify({a:special})).a);
  console.assert(Flatted.parse(Flatted.stringify({a:special})).a === special, 'no problem with simulation');
  special = "~\\x7e";
  console.assert(Flatted.parse(Flatted.stringify({a:special})).a === special, 'no problem with special char');
}());

(function () {
  var o = {a: 'a', b: 'b', c: function(){}, d: {e: 123}},
      a = JSON.stringify(o),
      b = Flatted.stringify(o);

  console.assert(
    JSON.stringify(JSON.parse(a)) === JSON.stringify(Flatted.parse(b)),
    'works as JSON.parse'
  );
  console.assert(
    Flatted.stringify(o, function(key, value){
      if (!key || key === 'a') return value;
    }) === '[{"a":"1"},"a"]',
    'accept callback'
  );
  console.assert(
    JSON.stringify(
      Flatted.parse('[{"a":"1"},"a"]', function(key, value){
        if (key === 'a') return 'b';
        return value;
      })
    ) === '{"a":"b"}',
    'revive callback'
  );
}());

(function () {
  var o = {}, before, after;
  o.a = o;
  o.c = {};
  o.d = {
    a: 123,
    b: o
  };
  o.c.e = o;
  o.c.f = o.d;
  o.b = o.c;
  before = Flatted.stringify(o);
  o = Flatted.parse(before);
  console.assert(
    o.b === o.c &&
    o.c.e === o &&
    o.d.a === 123 &&
    o.d.b === o &&
    o.c.f === o.d &&
    o.b === o.c,
    'recreated original structure'
  );
}());

(function () {
  var o = {};
  o.a = o;
  o.b = o;
  console.assert(
    Flatted.stringify(o, function (key, value) {
      if (!key || key === 'a') return value;
    }) === '[{"a":"0"}]',
    'callback invoked'
  );
  o = Flatted.parse('[{"a":"0"}]', function (key, value) {
    if (!key) {
      value.b = value;
    }
    return value;
  });
  console.assert(
    o.a === o && o.b === o,
    'reviver invoked'
  );
}());

(function () {
  var o = {};
  o['~'] = o;
  o['\\x7e'] = '\\x7e';
  o.test = '~';

  o = Flatted.parse(Flatted.stringify(o));
  console.assert(o['~'] === o && o.test === '~', 'still intact');
  o = {
    a: [
      '~', '~~', '~~~'
    ]
  };
  o.a.push(o);
  o.o = o;
  o['~'] = o.a;
  o['~~'] = o.a;
  o['~~~'] = o.a;
  o = Flatted.parse(Flatted.stringify(o));
  console.assert(
    o === o.a[3] &&
    o === o.o &&
    o['~'] === o.a &&
    o['~~'] === o.a &&
    o['~~~'] === o.a &&
    o.a === o.a[3].a &&
    o.a.pop() === o &&
    o.a.join('') === '~~~~~~',
    'restructured'
  );

}());

(function () {

  // make sure only own properties are parsed
  Object.prototype.shenanigans = true;

  var
    item = {
      name: 'TEST'
    },
    original = {
      outer: [
        {
          a: 'b',
          c: 'd',
          one: item,
          many: [item],
          e: 'f'
        }
      ]
    },
    str,
    output
  ;
  item.value = item;
  str = Flatted.stringify(original);
  output = Flatted.parse(str);
  console.assert(str === '[{"outer":"1"},["2"],{"a":"3","c":"4","one":"5","many":"6","e":"7"},"b","d",{"name":"8","value":"5"},["5"],"f","TEST"]', 'string is correct');
  console.assert(
    original.outer[0].one.name === output.outer[0].one.name &&
    original.outer[0].many[0].name === output.outer[0].many[0].name &&
    output.outer[0].many[0] === output.outer[0].one,
    'object too'
  );

  delete Object.prototype.shenanigans;

}());

(function () {
  var
    unique = {a:'sup'},
    nested = {
      prop: {
        value: 123
      },
      a: [
        {},
        {b: [
          {
            a: 1,
            d: 2,
            c: unique,
            z: {
              g: 2,
              a: unique,
              b: {
                r: 4,
                u: unique,
                c: 5
              },
              f: 6
            },
            h: 1
          }
        ]}
      ],
      b: {
        e: 'f',
        t: unique,
        p: 4
      }
    },
    str = Flatted.stringify(nested),
    output
  ;
  console.assert(str === '[{"prop":"1","a":"2","b":"3"},{"value":123},["4","5"],{"e":"6","t":"7","p":4},{},{"b":"8"},"f",{"a":"9"},["10"],"sup",{"a":1,"d":2,"c":"7","z":"11","h":1},{"g":2,"a":"7","b":"12","f":6},{"r":4,"u":"7","c":5}]', 'string is OK');
  output = Flatted.parse(str);
  console.assert(output.b.t.a === 'sup' && output.a[1].b[0].c === output.b.t, 'so is the object');
}());

(function () {
  var o = {bar: 'something ~ baz'};
  var s = Flatted.stringify(o);
  console.assert(s === '[{"bar":"1"},"something ~ baz"]', 'string is correct');
  var oo = Flatted.parse(s);
  console.assert(oo.bar === o.bar, 'parse is correct');
}());

(function () {
  var o = {};
  o.a = {
    aa: {
      aaa: 'value1'
    }
  };
  o.b = o;
  o.c = {
    ca: {},
    cb: {},
    cc: {},
    cd: {},
    ce: 'value2',
    cf: 'value3'
  };
  o.c.ca.caa = o.c.ca;
  o.c.cb.cba = o.c.cb;
  o.c.cc.cca = o.c;
  o.c.cd.cda = o.c.ca.caa;

  var s = Flatted.stringify(o);
  console.assert(s === '[{"a":"1","b":"0","c":"2"},{"aa":"3"},{"ca":"4","cb":"5","cc":"6","cd":"7","ce":"8","cf":"9"},{"aaa":"10"},{"caa":"4"},{"cba":"5"},{"cca":"2"},{"cda":"4"},"value2","value3","value1"]', 'string is correct');
  var oo = Flatted.parse(s);
  console.assert(
  oo.a.aa.aaa = 'value1'
    && oo === oo.b
    && oo.c.ca.caa === oo.c.ca
    && oo.c.cb.cba === oo.c.cb
    && oo.c.cc.cca === oo.c
    && oo.c.cd.cda === oo.c.ca.caa
    && oo.c.ce === 'value2'
    && oo.c.cf === 'value3',
    'parse is correct'
  );
}());

(function () {
  var
    original = {
      a1: {
        a2: [],
        a3: [{name: 'whatever'}]
      },
      a4: []
    },
    json,
    restored
  ;

  original.a1.a2[0] = original.a1;
  original.a4[0] = original.a1.a3[0];

  json = Flatted.stringify(original);
  restored = Flatted.parse(json);

  console.assert(restored.a1.a2[0] === restored.a1, '~a1~a2~0 === ~a1');
  console.assert(restored.a4[0] = restored.a1.a3[0], '~a4 === ~a1~a3~0');
}());

if (typeof Symbol !== 'undefined') {
  (function () {
    var o = {a: 1};
    var a = [1, Symbol('test'), 2];
    o[Symbol('test')] = 123;
    console.assert(('[' + JSON.stringify(o) + ']') === Flatted.stringify(o), 'Symbol is OK too');
    console.assert(('[' + JSON.stringify(a) + ']') === Flatted.stringify(a), 'non symbol is OK too');
  }());
}

(function () {
  var args = [{a:[1]}, null, '  '];
  console.assert(Flatted.stringify.apply(null, args) === "[{\n  \"a\": \"1\"\n},[\n  1\n]]", 'extra args same as JSON');
}());

(function () {
  var o = {a: 1, b: {a: 1, b: 2}};
  var json = JSON.stringify(o, ['b']);
  console.assert(
    Flatted.stringify(o, ['b']) === '[{"b":"1"},{"b":2}]',
    'whitelisted ["b"]: '+ json
  );
}());

(function () {
  var a = { b: { '': { c: { d: 1 } } } };
  a._circular = a.b[''];
  var json = Flatted.stringify(a);
  var nosj = Flatted.parse(json);
  console.assert(
    nosj._circular === nosj.b[''] &&
    JSON.stringify(nosj._circular) === JSON.stringify(a._circular),
    'empty keys as non root objects work'
  );
  delete a._circular;
  delete nosj._circular;
  console.assert(
    JSON.stringify(nosj) === JSON.stringify(a),
    'objects copied with circular empty keys are the same'
  );
}());

['65515.json', '65518.json'].forEach(fileName => {
  let dataString = require('fs').readFileSync('test/' + fileName).toString('utf-8');
  let rawJson = JSON.parse(dataString);
  let {toolData} = rawJson;
  console.assert(typeof Flatted.parse(JSON.stringify(toolData)) === 'object');
});

class RecursiveMap extends Map {
  static fromJSON(any) {
    return new this(Flatted.fromJSON(any));
  }
  toJSON() {
    return Flatted.toJSON([...this.entries()]);
  }
}

const jsonMap = new RecursiveMap([['test', 'value']]);
const asJSON = JSON.stringify(jsonMap);
const expected = '[["1"],["2","3"],"test","value"]';
console.assert(asJSON === expected, 'toJSON');
const revived = RecursiveMap.fromJSON(JSON.parse(asJSON));
console.assert(revived.get('test') === 'value', 'fromJSON');


const recursive = new RecursiveMap;
const same = {};
same.same = same;
recursive.set('same', same);

const asString = JSON.stringify(recursive);
const asMap = RecursiveMap.fromJSON(JSON.parse(asString));
console.assert(asMap.get('same').same === asMap.get('same'), 'RecursiveMap');
