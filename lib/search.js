function lines(corpus) {
  return corpus.split(/\n+/)
               .map(function(line) {
                 return line.substring(nthIndexOf(line, ",", 3) + 1);
               });
}

function nthIndexOf(haystack, needle, n) {
  var index = -1;
  for (var i = 0; i < n; i++) {
    index = haystack.indexOf(needle, index + 1);
    if (index < 0) {
      return -1;
    }
  }
  return index;
}

function skipPunc(word) {
  for (var i = 0, n = word.length; i < n; i++) {
    if (/[a-zA-Z]/.test(word[i])) {
      break;
    }
  }
  return word.substring(i);
}

function matches(word, search) {
  var start = skipPunc(word);
  var i = 0, m = start.length, n = search.length;
  if (m < n) {
    return false;
  }
  while (i < n) {
    if (start[i].toLowerCase() !== search[i]) {
      return false;
    }
    i++;
  }
  return i == m || !/[a-zA-Z]/.test(start[i]);
}

function wcLine(line, search) {
  var words = line.split(' ');
  var total = 0;
  for (var i = 0, n = words.length; i < n; i++) {
    if (matches(words[i], search)) {
      total++;
    }
  }
  return total;
}

exports.search = function search(corpus, search) {
  var ls = lines(corpus);
  var total = 0;
  for (var i = 0, n = ls.length; i < n; i++) {
    total += wcLine(ls[i], search);
  }
  return total;
}
