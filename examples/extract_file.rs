//! An example of extracting a file in an archive.
//!
//! Takes a tarball on standard input, looks for an entry with a listed file
//! name as the first argument provided, and then prints the contents of that
//! file to stdout.

extern crate tar;

use async_std::io::{copy, stdin, stdout};
use async_std::path::Path;
use async_std::prelude::*;
use std::env::args_os;

use tar::Archive;

fn main() {
    async_std::task::block_on(async {
        let first_arg = args_os().skip(1).next().unwrap();
        let filename = Path::new(&first_arg);
        let mut ar = Archive::new(stdin());
        while let Some(file) = ar.entries().unwrap().next().await {
            let mut f = file.unwrap();
            if f.path().unwrap() == filename {
                copy(&mut f, &mut stdout()).await.unwrap();
            }
        }
    });
}
