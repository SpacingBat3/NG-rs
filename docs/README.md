<!--
SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>

SPDX-License-Identifier: GPL-3.0-or-later
-->

> [!IMPORTANT]
> This project is currently at prototype / initial stage. Before using
> it, be aware of major API breakages happening continuously and
> that overall API is highly unstable and requires proper centralized
> organization.
> A lot of stuff, including naming and namespace organization
> conventions, are a subject to change. In fact, most structures have
> private access to additionaly discourage relying on unstable APIs.

<div align=center>

NG-rs
=====================================================================

Bringing Rust into [Newgrounds]!

</div>

About
---------------------------------------------------------------------

This project focuses on providing a complete ecosystem for Rust, that
can allow for various interactions with [Newgrounds], and to aid
developers to build applications – hopefully that are going to be
respectful towards NG. It is still unstable, with most of APIs
being a POC than something that can be trusted for any applications,
and requires polishing and possibly external developers feedback
for applications that are not part of my use case scenarios.

Unless where official API is being provided, this uses scrapping &
proof-of-work solving to offer browser-grade experience, with intent
on various use cases in native interactive applications.

### Goals

> [!NOTE]
> Many of these are still in-progress, due to library being immature
> and still subject to major changes – e.g. `mpsc` from `std` can
> be replaced with async-friendly variant to allow main threads
> to do tasks than being blocked.

- **Be respectful**: api consumers utilizing this library should not
  do any greater harm than usual users browsing.

- **Be a native-grade implementation**: library should aim for much
  better OS resources utilization than web, rely on compile-time
  optimizations and static typing for optimizations, and offer
  more opportunities for client to declare how to exactly utilize
  resources and for what purpose, by defining data memory layout.

- **Async I/O management**: if possible, tasks should avoid entirely
  blocking the thread and utilize async API to (optionally) allow for
  further execution.

- **Be extremely tunable**: anyone using this library should be able
  to have version tuned to their use cases at compile time, by
  disabling APIs they don't need to support or switching backends for
  the existing ones via `[feature]` configurations.

### Non-requirements

- **Less strain on the server than web-browsers:** while not
  specifically focusing on the server utilization efficiency, this
  might be a consequence of more shallow resource fetching than web
  browser, and with more mature cookies and session handling, there
  might be need to request less resources from the server than web
  browsers usually do.

Components
---------------------------------------------------------------------

Currently, most of the components are WIP:

- `ng_rs`: modular library

  - `ng_rs-common`: common definitions
  - `ng_rs-aud`: Audio API
  - `ng_rs-guard` Guard API

- `ng_rs-aud-cli` A Newgrounds CLI audio browse tool

[Newgrounds]: https://www.newgrounds.com "Everything, by everyone"
