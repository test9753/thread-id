// Thread-ID -- Get a unique thread ID
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! Thread-ID: get a unique ID for the current thread.
//!
//! # Example
//!
//! ```
//! use std::thread;
//! use thread_id;
//!
//! thread::spawn(move || {
//!     println!("spawned thread has id {}", thread_id::get());
//! });
//!
//! println!("main thread has id {}", thread_id::get());
//! ```

#![warn(missing_docs)]

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate kernel32;

/// Returns a number that is unique per thread.
pub fn get() -> u64 {
    get_internal()
}

#[cfg(unix)]
fn get_internal() -> u64 {
    unsafe { libc::pthread_self() as u64 }
}

#[cfg(windows)]
fn get_internal() -> u64 {
    unsafe { kernel32::GetCurrentThreadId() as u64 }
}

#[test]
fn distinct_threads_have_distinct_ids() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(::get()).unwrap()).join().unwrap();

    let main_tid = ::get();
    let other_tid = rx.recv().unwrap();
    assert!(main_tid != other_tid);
}
