var Flatted = (function (String) {

  /*! (c) 2018 Andrea Giammarchi - (ISC) */

  var Flatted = {

    parse: function parse(text) {
      var input = JSON.parse(text, Strings);
      return revive(input.map(strings), new Set, input[0]);
    },

    stringify: function stringify(value) {
      for (var
        firstRun,
        i = 0,
        known = new Map,
        output = (known.set(value, '' + i), []),
        input = [value],
        replace = function (key, value) {
          if (firstRun) return (firstRun = !firstRun), value;
          switch (typeof value) {
            case "object":
              if (value === null) return value;
            case "string":
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
        if (value instanceof String) {
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
    var index = '' + (input.push(value) - 1);
    known.set(value, index);
    return index;
  }

  function strings(value) {
    return value instanceof String ? ('' + value) : value;
  }

  function Strings(key, value) {
    return typeof value === 'string' ? new String(value) : value;
  }

}(String));
