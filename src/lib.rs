// Copyright 2019 Q1t BV
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)] #[macro_use] extern crate matches;
#[macro_use] extern crate ndarray;

#[macro_use] pub mod cmatrix;
#[macro_use] pub mod gates;
pub mod circuit;
pub mod error;
pub mod export;
pub mod permutation;
pub mod qustate;

mod idhash;
mod support;
#[cfg(test)] mod stats;
