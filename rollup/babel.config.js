import babel from 'rollup-plugin-babel';

export default {
  input: './esm/index.js',
  plugins: [
    babel({presets: ['@babel/preset-env']})
  ],
  output: {
    esModule: false,
    exports: 'named',
    file: './index.js',
    format: 'iife',
    name: 'Flatted'
  }
};
