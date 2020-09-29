const fs = require('fs');
const {parse} = require('../');

['65515.json', '65518.json'].forEach(fileName => {
  let dataString = fs.readFileSync('test/' + fileName).toString('utf-8');
  let rawJson = JSON.parse(dataString);
  let {toolData} = rawJson;

  let stringified = parse(JSON.stringify(toolData));

  console.log(stringified);
});
