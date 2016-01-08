#[macro_use]
extern crate neon;
extern crate rayon;

use rayon::par_iter::{ParallelIterator, IntoParallelIterator};

use neon::vm;
use neon::vm::{Call, JsResult};
use neon::js::{JsInteger, JsString};
use neon::js::binary::JsBuffer;
use neon::mem::Handle;

fn lines(corpus: &str) -> Vec<&str> {
    corpus.lines()
          .map(|line| {
              line.splitn(4, ',').nth(3).unwrap().trim()
          })
          .collect()
}

fn matches(word: &str, search: &str) -> bool {
    let mut search = search.chars();
    for ch in word.chars().skip_while(|ch| !ch.is_alphabetic()) {
        match search.next() {
            None => { return !ch.is_alphabetic(); }
            Some(expect) => {
                if ch.to_lowercase().next() != Some(expect) {
                    return false;
                }
            }
        }
    }
    return search.next().is_none();
}

fn wc_line(line: &str, search: &str) -> i32 {
    let mut total = 0;
    for word in line.split(' ') {
        if matches(word, search) {
            total += 1;
        }
    }
    total
}

// Also valid, with comparable performance:

/*
fn wc_line(line: &str, search: &str) -> i32 {
    line.split(' ')
        .filter(|word| matches(word, search))
        .fold(0, |sum, _| sum + 1)
}
*/

fn wc_sequential(lines: &Vec<&str>, search: &str) -> i32 {
    lines.into_iter()
         .map(|line| wc_line(line, search))
         .fold(0, |sum, line| sum + line)
}

fn wc_parallel(lines: &Vec<&str>, search: &str) -> i32 {
    lines.into_par_iter()
         .map(|line| wc_line(line, search))
         .sum()
}

fn search(call: Call) -> JsResult<JsInteger> {
    let scope = call.scope;
    let buffer: Handle<JsBuffer> = try!(try!(call.arguments.require(scope, 0)).check::<JsBuffer>());
    let string: Handle<JsString> = try!(try!(call.arguments.require(scope, 1)).check::<JsString>());
    let search = &string.data()[..];
    let total = vm::lock(buffer, |data| {
        let corpus = data.as_str().unwrap();
        wc_parallel(&lines(corpus), search)
    });
    Ok(JsInteger::new(scope, total))
}

register_module!(m, {
    m.export("search", search)
});
