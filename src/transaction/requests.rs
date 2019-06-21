// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use super::Transaction;
use crate::{Error, Key, KvPair, Value};

use futures::prelude::*;
use futures::task::{Context, Poll};
use std::pin::Pin;

/// An unresolved [`Transaction::scan`](Transaction::scan) request.
///
/// Once resolved this request will result in a scanner over the given keys.
pub struct Scanner;

impl Stream for Scanner {
    type Item = Result<KvPair, Error>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<Self::Item>> {
        unimplemented!()
    }
}

/// An unresolved [`Transaction::get`](Transaction::get) request.
///
/// Once resolved this request will result in the fetching of the value associated with the given
/// key.
pub struct Get {
    key: Key,
}

impl Get {
    pub fn new(key: Key) -> Self {
        Get { key }
    }
}

impl Future for Get {
    type Output = Result<Value, Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        let _key = &self.key;
        unimplemented!()
    }
}

/// An unresolved [`Transaction::batch_get`](Transaction::batch_get) request.
///
/// Once resolved this request will result in the fetching of the values associated with the given
/// keys.
pub struct BatchGet {
    keys: Vec<Key>,
}

impl BatchGet {
    pub fn new(keys: Vec<Key>) -> Self {
        BatchGet { keys }
    }
}

impl Future for BatchGet {
    type Output = Result<Vec<KvPair>, Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        let _keys = &self.keys;
        unimplemented!()
    }
}

/// An unresolved [`Transaction::commit`](Transaction::commit) request.
///
/// Once resolved this request will result in the committing of the transaction.
pub struct Commit {
    txn: Transaction,
}

impl Commit {
    pub fn new(txn: Transaction) -> Self {
        Commit { txn }
    }
}

impl Future for Commit {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        let _txn = &self.txn;
        unimplemented!()
    }
}

/// An unresolved [`Transaction::rollback`](Transaction::rollback) request.
///
/// Once resolved this request will result in the rolling back of the transaction.
pub struct Rollback {
    txn: Transaction,
}

impl Rollback {
    pub fn new(txn: Transaction) -> Self {
        Rollback { txn }
    }
}

impl Future for Rollback {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        let _txn = &self.txn;
        unimplemented!()
    }
}

/// An unresolved [`Transaction::lock_keys`](Transaction::lock_keys) request.
///
/// Once resolved this request will result in the locking of the given keys.
pub struct LockKeys {
    keys: Vec<Key>,
}

impl LockKeys {
    pub fn new(keys: Vec<Key>) -> Self {
        LockKeys { keys }
    }
}

impl Future for LockKeys {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        let _keys = &self.keys;
        unimplemented!()
    }
}

/// An unresolved [`Transaction::set`](Transaction::set) request.
///
/// Once resolved this request will result in the setting of the value associated with the given
/// key.
pub struct Set {
    key: Key,
    value: Value,
}

impl Set {
    pub fn new(key: Key, value: Value) -> Self {
        Set { key, value }
    }
}

impl Future for Set {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        let _key = &self.key;
        let _value = &self.value;
        unimplemented!()
    }
}

/// An unresolved [`Transaction::delete`](Transaction::delete) request.
///
/// Once resolved this request will result in the deletion of the given key.
pub struct Delete {
    key: Key,
}

impl Delete {
    pub fn new(key: Key) -> Self {
        Delete { key }
    }
}

impl Future for Delete {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        let _key = &self.key;
        unimplemented!()
    }
}