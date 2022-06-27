import {terser} from 'rollup-plugin-terser';

export default {
  input: './esm/index.js',
  plugins: [
    terser()
  ],
  output: {
    file: './esm.js',
    format: 'module'
  }
};
