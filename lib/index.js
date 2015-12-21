var fs = require('fs');
var path = require('path');
var bench = require('./bench');

var wc = {
  js: require('./search'),
  neon: require('neon-bridge')()
};

var ROOT = path.resolve(__dirname, "..");
var DATA = path.resolve(ROOT, "data");

var string = fs.readFileSync(path.resolve(DATA, "shakespeare-plays.csv"), 'utf8');
var buffer = fs.readFileSync(path.resolve(DATA, "shakespeare-plays.csv"));

console.log(bench(function() { return wc.js.search(string, "thee"); }));
console.log(bench(function() { return wc.neon.search(buffer, "thee"); }));
