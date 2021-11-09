// original file from:
// https://github.com/yyx990803/circular-json-es6

var dummy100 = require('./data.json');
var dummy50 = dummy100.slice(0, 50)
var dummy10 = dummy100.slice(0, 10)

function bench(method, dummy) {
  var t = Date.now(), i = 0
  while ((Date.now() - t) < 1000) {
    r = method(dummy)
    i++
  }
  return i
}

var r
var CircularJSON = require('circular-json')
var jsan = require('jsan')
var cj6 = require('circular-json-es6')
var flatted = require('../cjs')
var SC = require('@ungap/structured-clone/json')

function run(name, fn, dummy) {
  console.log(
    name + ' ' + (
      typeof dummy === 'string' ?
        dummy.length + ' chars' :
        (dummy.length || '') + ' objects'
    ) + ' parsed ' + (
      bench(fn, dummy) / 1000
    ).toFixed(2) + ' times per second'
  )
}

//*
console.log('-----------------------------------')
console.log('Object with ' + Object.keys(dummy100[0]).length + ' keys each')
console.log('-----------------------------------')
run('CircularJSON', CircularJSON.stringify, dummy100)
run('CircularJSON', CircularJSON.parse, r)
run('CircularJSON', CircularJSON.stringify, dummy50)
run('CircularJSON', CircularJSON.parse, r)
run('CircularJSON', CircularJSON.stringify, dummy10)
run('CircularJSON', CircularJSON.parse, r)
console.log('-----------------------------------')
run('circular-json-es6', cj6.stringify, dummy100)
run('circular-json-es6', cj6.parse, r)
run('circular-json-es6', cj6.stringify, dummy50)
run('circular-json-es6', cj6.parse, r)
run('circular-json-es6', cj6.stringify, dummy10)
run('circular-json-es6', cj6.parse, r)
console.log('-----------------------------------')
run('jsan', jsan.stringify, dummy100)
run('jsan', jsan.parse, r)
run('jsan', jsan.stringify, dummy50)
run('jsan', jsan.parse, r)
run('jsan', jsan.stringify, dummy10)
run('jsan', jsan.parse, r)
console.log('-----------------------------------')
run('flatted', flatted.stringify, dummy100)
run('flatted', flatted.parse, r)
run('flatted', flatted.stringify, dummy50)
run('flatted', flatted.parse, r)
run('flatted', flatted.stringify, dummy10)
run('flatted', flatted.parse, r)
console.log('-----------------------------------')
run('Structured Clone', SC.stringify, dummy100)
run('Structured Clone', SC.parse, r)
run('Structured Clone', SC.stringify, dummy50)
run('Structured Clone', SC.parse, r)
run('Structured Clone', SC.stringify, dummy10)
run('Structured Clone', SC.parse, r)
console.log('-----------------------------------')
console.log('50% same objects')
dummy100 = dummy50.concat(dummy50)
console.log('-----------------------------------')
run('CircularJSON', CircularJSON.stringify, dummy100)
run('CircularJSON', CircularJSON.parse, r)
run('circular-json-es6', cj6.stringify, dummy100)
run('circular-json-es6', cj6.parse, r)
run('jsan', jsan.stringify, dummy100)
run('jsan', jsan.parse, r)
run('flatted', flatted.stringify, dummy100)
run('flatted', flatted.parse, r)
run('Structured Clone', SC.stringify, dummy100)
run('Structured Clone', SC.parse, r)
console.log('-----------------------------------')
console.log('90% same objects')
dummy100 = [].concat(
  dummy10, dummy10, dummy10, dummy10, dummy10,
  dummy10, dummy10, dummy10, dummy10, dummy10
)
console.log('-----------------------------------')
run('CircularJSON', CircularJSON.stringify, dummy100)
run('CircularJSON', CircularJSON.parse, r)
run('circular-json-es6', cj6.stringify, dummy100)
run('circular-json-es6', cj6.parse, r)
run('jsan', jsan.stringify, dummy100)
run('jsan', jsan.parse, r)
run('flatted', flatted.stringify, dummy100)
run('flatted', flatted.parse, r)
run('Structured Clone', SC.stringify, dummy100)
run('Structured Clone', SC.parse, r)
console.log('-----------------------------------')
console.log('with circular')
function makeCircularObject () {
  var a = {}
  a.b = a
  return a
}
dummy100 = []
for (var i = 0; i < 100; i++) {
  dummy100.push(makeCircularObject())
}
console.log('-----------------------------------')
run('CircularJSON', CircularJSON.stringify, dummy100)
run('CircularJSON', CircularJSON.parse, r)
run('circular-json-es6', cj6.stringify, dummy100)
run('circular-json-es6', cj6.parse, r)
run('jsan', jsan.stringify, dummy100)
run('jsan', jsan.parse, r)
run('flatted', flatted.stringify, dummy100)
run('flatted', flatted.parse, r)
run('Structured Clone', SC.stringify, dummy100)
run('Structured Clone', SC.parse, r)
console.log('-----------------------------------')
console.log('with circular 90% same')
function makeCircularObject () {
  var a = {}
  a.b = a
  return a
}
dummy10 = []
for (var i = 0; i < 10; i++) {
  dummy10.push(makeCircularObject())
}
dummy100 = [].concat(
  dummy10, dummy10, dummy10, dummy10, dummy10,
  dummy10, dummy10, dummy10, dummy10, dummy10
)
console.log('-----------------------------------')
run('CircularJSON', CircularJSON.stringify, dummy100)
run('CircularJSON', CircularJSON.parse, r)
run('circular-json-es6', cj6.stringify, dummy100)
run('circular-json-es6', cj6.parse, r)
run('jsan', jsan.stringify, dummy100)
run('jsan', jsan.parse, r)
run('flatted', flatted.stringify, dummy100)
run('flatted', flatted.parse, r)
run('Structured Clone', SC.stringify, dummy100)
run('Structured Clone', SC.parse, r)
// */
console.log('-----------------------------------')
console.log('Big real-world circular data')
var cirular = CircularJSON.parse(
              require('fs').readFileSync(
                require('path').join(__dirname, './circular.txt')
              ).toString());
console.log('-----------------------------------')
run('CircularJSON', CircularJSON.stringify, cirular)
run('CircularJSON', CircularJSON.parse, r)
run('circular-json-es6', cj6.stringify, cirular)
run('circular-json-es6', cj6.parse, r)
run('jsan', jsan.stringify, cirular)
run('jsan', jsan.parse, r)
run('flatted', flatted.stringify, cirular)
run('flatted', flatted.parse, r)
run('Structured Clone', SC.stringify, cirular)
run('Structured Clone', SC.parse, r)
