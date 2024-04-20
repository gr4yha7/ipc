// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

 use fil_actors_runtime::actor_dispatch;
 use fil_actors_runtime::actor_error;
 use fil_actors_runtime::builtin::singletons::SYSTEM_ACTOR_ADDR;
 use fil_actors_runtime::builtin::singletons;
 use fil_actors_runtime::runtime::{ActorCode, Runtime};
 use fil_actors_runtime::ActorError;

 use crate::{Method, CUSTOMSYSCALL_ACTOR_NAME};

 fil_actors_runtime::wasm_trampoline!(Actor);

 fvm_sdk::sys::fvm_syscalls! {
     module = "chess_engine_kernel";
     pub fn chess_engine_syscall() -> Result<u64>;
 }

 pub struct Actor;
 impl Actor {
     fn invoke(rt: &impl Runtime) -> Result<u64, ActorError> {
         rt.validate_immediate_caller_is(std::iter::once(&SYSTEM_ACTOR_ADDR))?;

         unsafe {
             let value = chess_engine_syscall().unwrap();
             Ok(value)
         }
     }
 }

 impl ActorCode for Actor {
     type Methods = Method;

     fn name() -> &'static str {
         CHESS_ENGINE_SYSCALL_ACTOR_NAME
     }

     actor_dispatch! {
         Invoke => invoke,
     }
 }