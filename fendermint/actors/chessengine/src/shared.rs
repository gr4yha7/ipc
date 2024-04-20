// Copyright 2021-2023 Protocol Labs
 // SPDX-License-Identifier: Apache-2.0, MIT

 use num_derive::FromPrimitive;

 pub const CHESS_ENGINE_SYSCALL_ACTOR_NAME: &str = "chess_engine_syscall";

 #[derive(FromPrimitive)]
 #[repr(u64)]
 pub enum Method {
     Invoke = frc42_dispatch::method_hash!("Invoke"),
 }