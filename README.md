# flatted

[![Coverage Status](https://coveralls.io/repos/github/WebReflection/flatted/badge.svg?branch=master)](https://coveralls.io/github/WebReflection/flatted?branch=master) [![Build Status](https://travis-ci.org/WebReflection/flatted.svg?branch=master)](https://travis-ci.org/WebReflection/flatted) [![License: ISC](https://img.shields.io/badge/License-ISC-yellow.svg)](https://opensource.org/licenses/ISC)

A fast and minimal circular JSON parser.

Usable via [CDN](https://unpkg.com/flatted) or as regular module.

```js
// ESM
import {parse, stringify} from 'flatted/esm';

// CJS
const {parse, stringify} = require('flatted/cjs');

const a = [{}];
a[0].a = a;
a.push(a);

stringify(a); // [["1","0"],{"a":"0"}]
```


### Compatibility
All ECMAScript engines compatible with `Map`, `Set`, `Object.keys`, and `Array.prototype.reduce` will work, even if polyfilled.


### How does it work ?
While stringifying, all Objects, including Arrays, and strings, are flattened out and replaced as unique index. `*`

Once parsed, all indexes will be replaced through the flattened collection.

<sup><sub>`*` represented as string to avoid conflicts with numbers</sub></sup>

```js
// logic example
var a = [{one: 1}, {two: '2'}];
a[0].a = a;
// a is the main object, will be at index '0'
// {one: 1} is the second object, index '1'
// {two: '2'} the third, in '2', and it has a string
// which will be found at index '3'

Flatted.stringify(a);
// [["1","2"],{"one":1,"a":"0"},{"two":"3"},"2"]
// a[one,two]    {one: 1, a}    {two: '2'}  '2'
```


### Why not CircularJSON ?
If approved by the community, this project will be the core of [CircularJSON](https://github.com/WebReflection/circular-json/#circularjson) V2.

The main reasons for having this software a part are:

  * battle-testing against all complex cases before releasing CircularJSON V2
  * provide a minimal parser without extra complexity needed by revivers and replacers
  * give developers a way to compare the two formats (on average, with big structures, smaller with flatted)

Once satisfied by feedbacks and results, there will be a version of this project fully compatible with JSON API.
