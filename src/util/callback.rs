// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
// Copyright (c) 2023 lizidev
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
// MIT License, taken from https://github.com/thaw-ui/thaw at 69bc65d
use std::{ops::Deref, sync::Arc};

pub struct BoxCallback(Box<dyn Fn() + Send + Sync + 'static>);

impl BoxCallback {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self(Box::new(f))
    }
}

impl Deref for BoxCallback {
    type Target = Box<dyn Fn() + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> From<F> for BoxCallback
where
    F: Fn() + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

pub struct BoxOneCallback<A, Return = ()>(Box<dyn Fn(A) -> Return + Send + Sync + 'static>);

impl<A, Return> BoxOneCallback<A, Return> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> Return + Send + Sync + 'static,
    {
        Self(Box::new(f))
    }
}

impl<A, Return> Deref for BoxOneCallback<A, Return> {
    type Target = Box<dyn Fn(A) -> Return + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, A, Return> From<F> for BoxOneCallback<A, Return>
where
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

#[derive(Clone)]
pub struct ArcCallback(Arc<dyn Fn() + Send + Sync + 'static>);

impl ArcCallback {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }
}

impl Deref for ArcCallback {
    type Target = Arc<dyn Fn() + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> From<F> for ArcCallback
where
    F: Fn() + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

pub struct ArcOneCallback<A, Return = ()>(Arc<dyn Fn(A) -> Return + Send + Sync + 'static>);

impl<K, V> Clone for ArcOneCallback<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A, Return> ArcOneCallback<A, Return> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> Return + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }
}

impl<A, Return> Deref for ArcOneCallback<A, Return> {
    type Target = Arc<dyn Fn(A) -> Return + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, A, Return> From<F> for ArcOneCallback<A, Return>
where
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

#[derive(Clone)]
pub struct ArcTwoCallback<A, B>(Arc<dyn Fn(A, B) + Send + Sync + 'static>);

impl<A, B> ArcTwoCallback<A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A, B) + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }
}

impl<A, B> Deref for ArcTwoCallback<A, B> {
    type Target = Arc<dyn Fn(A, B) + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, A, B> From<F> for ArcTwoCallback<A, B>
where
    F: Fn(A, B) + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}
