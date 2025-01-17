// Copyright 2022 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The internal implement details of OpenDAL.
//!
//! OpenDAL has provides unified abstraction via two-level API sets:
//!
//! - Public API like [`Operator`], [`Object`] provides user level API.
//! - Raw API like [`Accessor`], [`Layer`] provides developer level API.
//!
//! OpenDAL tries it's best to keep the public API stable. But raw APIs
//! may change between minor releases from time to time. So most users
//! should only use the public API. And only developers need to implement
//! with raw API to implement a new service [`Accessor`] or their own
//! [`Layer`].
//!
//! In this section, we will talk about the following components:
//!
//! - [`Accessor`][accessor]: to connect underlying storage services.
//! - [`Layer`][layer]: middleware/interceptor between storage services.
//!
//! The relation between [`Accessor`], [`Layer`] and [`Operator`] looks like the following:
//!
//! ```text
//! ┌─────────────────────────────────────────────────┬──────────┐
//! │                                                 │          │
//! │              ┌──────────┐  ┌────────┐           │          │
//! │              │          │  │        ▼           │          │
//! │      s3──┐   │          │  │ Tracing Layer      │          │
//! │          │   │          │  │        │           │          │
//! │     gcs──┤   │          │  │        ▼           │          │
//! │          ├──►│ Accessor ├──┘ Metrics Layer ┌───►│ Operator │
//! │  azblob──┤   │          │           │      │    │          │
//! │          │   │          │           ▼      │    │          │
//! │    hdfs──┘   │          │    Logging Layer │    │          │
//! │              │          │           │      │    │          │
//! │              └──────────┘           └──────┘    │          │
//! │                                                 │          │
//! └─────────────────────────────────────────────────┴──────────┘
//! ```
//!
//! [`Builder`]: crate::Builder
//! [`Operator`]: crate::Operator
//! [`Object`]: crate::Object
//! [`Accessor`]: crate::raw::Accessor
//! [`Layer`]: crate::raw::Layer

pub mod accessor;
pub mod layer;
