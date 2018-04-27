var Flatted = (function (Primitive, primitive) {

  /*!
   * ISC License
   *
   * Copyright (c) 2018, Andrea Giammarchi, @WebReflection
   *
   * Permission to use, copy, modify, and/or distribute this software for any
   * purpose with or without fee is hereby granted, provided that the above
   * copyright notice and this permission notice appear in all copies.
   *
   * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
   * REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
   * AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
   * INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
   * LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE
   * OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
   * PERFORMANCE OF THIS SOFTWARE.
   */

  var Flatted = {

    parse: function parse(text) {
      var input = JSON.parse(text, Primitives).map(primitives);
      var value = input[0];
      return typeof value === 'object' && value ?
              revive(input, new Set, value) : value;
    },

    stringify: function stringify(value) {
      for (var
        firstRun,
        known = new Map,
        input = [],
        output = [],
        i = +set(known, input, value),
        replace = function (key, value) {
          if (firstRun) return (firstRun = !firstRun), value;
          switch (typeof value) {
            case 'object':
              if (value === null) return value;
            case primitive:
              return known.get(value) || set(known, input, value);
          }
          return value;
        };
        i < input.length; i++
      ) {
        firstRun = true;
        output[i] = JSON.stringify(input[i], replace);
      }
      return '[' + output.join(',') + ']';
    }

  };

  return Flatted;

  function revive(input, parsed, output) {
    return Object.keys(output).reduce(
      function (output, key) {
        var value = output[key];
        if (value instanceof Primitive) {
          var tmp = input[value];
          if (typeof tmp === 'object' && !parsed.has(tmp)) {
            parsed.add(tmp);
            output[key] = revive(input, parsed, tmp);
          } else {
            output[key] = tmp;
          }
        }
        return output;
      },
      output
    );
  }

  function set(known, input, value) {
    var index = Primitive(input.push(value) - 1);
    known.set(value, index);
    return index;
  }

  function primitives(value) {
    return value instanceof Primitive ? Primitive(value) : value;
  }

  function Primitives(key, value) {
    return typeof value === primitive ? new Primitive(value) : value;
  }

}(String, 'string'));
