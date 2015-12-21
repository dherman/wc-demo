var hrtime = process.hrtime;

function Benchmark(result, diff) {
  this.result = result;
  this.elapsed = diff;
  this.sec = diff[0];
  this.msec = Math.floor(diff[1] / 1000000);
  this.μsec = Math.floor((diff[1] - this.msec * 1000000) / 1000);
  this.nsec = diff[1] - (this.msec * 1000000) - (this.μsec * 1000);
}

Benchmark.of = function(thunk) {
  var time = hrtime();
  var result = thunk();
  var diff = hrtime(time);
  return new Benchmark(result, diff);
};

Benchmark.prototype.toString = function() {
  var components = [];
  if (this.sec) {
    components.push(this.sec + "s");
  }
  if (this.msec) {
    components.push(this.msec + "ms");
  }
  if (this.μsec) {
    components.push(this.μsec + "μs");
  }
  if (this.nsec) {
    components.push(this.nsec + "ns");
  }
  return JSON.stringify(this.result) + " [" + components.join(", ") + "]";
};

Benchmark.prototype.inspect = Benchmark.prototype.toString;

module.exports = Benchmark.of;
