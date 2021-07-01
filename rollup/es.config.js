import {terser} from 'rollup-plugin-terser';

export default {
  input: './esm/index.js',
  plugins: [
    terser()
  ],

  output: {
    esModule: false,
    exports: 'named',
    file: './es.js',
    format: 'iife',
    name: 'Flatted'
  }
};
