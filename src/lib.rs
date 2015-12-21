extern crate neon;
extern crate rayon;

use rayon::par_iter::{ParallelIterator, IntoParallelIterator};

use neon::vm;
use neon::vm::{Call, Result, JS, Module};
use neon::value::{Integer, String};
use neon::mem::Handle;
use neon::buffer::Buffer;

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
    let mut total: i32 = 0;
    for word in line.split(' ') {
        if matches(word, search) {
            total += 1;
        }
    }
    total
}

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

fn search(call: Call) -> JS<Integer> {
    let scope = call.scope;
    let buffer: Handle<Buffer> = try!(try!(call.arguments.require(scope, 0)).check::<Buffer>());
    let string: Handle<String> = try!(try!(call.arguments.require(scope, 1)).check::<String>());
    let search = &string.data()[..];
    let total = vm::lock(buffer, |data| {
        let corpus = data.as_str().unwrap();
        wc_parallel(&lines(corpus), search)
    });
    Ok(Integer::new(scope, total))
}

#[no_mangle]
pub fn node_main(mut module: Module) -> Result<()> {
    module.export("search", search)
}
