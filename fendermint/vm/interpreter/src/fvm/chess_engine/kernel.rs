use anyhow::Ok;
// Copyright 2021-2023 Protocol Labs
 // SPDX-License-Identifier: Apache-2.0, MIT
 use fvm::call_manager::CallManager;
 use fvm::gas::Gas;
 use fvm::kernel::prelude::*;
 use fvm::kernel::Result;
 use fvm::kernel::{
     ActorOps, CryptoOps, DebugOps, EventOps, IpldBlockOps, MessageOps, NetworkOps, RandomnessOps,
     SelfOps, SendOps, SyscallHandler, UpgradeOps,
 };
 use fvm::syscalls::Linker;
 use fvm::DefaultKernel;
 use fvm_shared::clock::ChainEpoch;
 use fvm_shared::randomness::RANDOMNESS_LENGTH;
 use fvm_shared::sys::out::network::NetworkContext;
 use fvm_shared::sys::out::vm::MessageContext;
 use fvm_shared::{address::Address, econ::TokenAmount, ActorID, MethodNum};

 use ambassador::Delegate;
 use cid::Cid;
 use fendermint_vm_chess_engine::engine;

 // we define a single custom syscall which simply doubles the input
 pub trait ChessEngineKernel: Kernel {
     fn chess_engine_syscall(&self) -> Result<u64>;
 }

 // our custom kernel extends the filecoin kernel
 #[derive(Delegate)]
 #[delegate(IpldBlockOps, where = "C: CallManager")]
 #[delegate(ActorOps, where = "C: CallManager")]
 #[delegate(CryptoOps, where = "C: CallManager")]
 #[delegate(DebugOps, where = "C: CallManager")]
 #[delegate(EventOps, where = "C: CallManager")]
 #[delegate(MessageOps, where = "C: CallManager")]
 #[delegate(NetworkOps, where = "C: CallManager")]
 #[delegate(RandomnessOps, where = "C: CallManager")]
 #[delegate(SelfOps, where = "C: CallManager")]
 #[delegate(SendOps<K>, generics = "K", where = "K: ChessEngineKernel")]
 #[delegate(UpgradeOps<K>, generics = "K", where = "K: ChessEngineKernel")]
 pub struct ChessEngineKernelImpl<C>(pub DefaultKernel<C>);

 impl<C> ChessEngineKernel for ChessEngineKernelImpl<C>
 where
     C: CallManager,
     ChessEngineKernelImpl<C>: Kernel,
 {
     fn chess_engine_syscall(&self) -> Result<u64> {
         // Here we have access to the Kernel structure and can call
         // any of its methods, send messages, etc.

         // We can also run an external program, link to any rust library
         // access the network, etc.

         // In this example, lets access the file system and return
         // the number of paths in /
        //  let paths = std::fs::read_dir("/").unwrap();
        //  Ok(paths.count() as u64)

         let chess_engine = engine::init();
         Ok(1u64)
     }
 }

 impl<C> Kernel for ChessEngineKernelImpl<C>
 where
     C: CallManager,
 {
     type CallManager = C;
     type Limiter = <DefaultKernel<C> as Kernel>::Limiter;

     fn into_inner(self) -> (Self::CallManager, BlockRegistry)
     where
         Self: Sized,
     {
         self.0.into_inner()
     }

     fn new(
         mgr: C,
         blocks: BlockRegistry,
         caller: ActorID,
         actor_id: ActorID,
         method: MethodNum,
         value_received: TokenAmount,
         read_only: bool,
     ) -> Self {
         ChessEngineKernelImpl(DefaultKernel::new(
             mgr,
             blocks,
             caller,
             actor_id,
             method,
             value_received,
             read_only,
         ))
     }

     fn machine(&self) -> &<Self::CallManager as CallManager>::Machine {
         self.0.machine()
     }

     fn limiter_mut(&mut self) -> &mut Self::Limiter {
         self.0.limiter_mut()
     }

     fn gas_available(&self) -> Gas {
         self.0.gas_available()
     }

     fn charge_gas(&self, name: &str, compute: Gas) -> Result<GasTimer> {
         self.0.charge_gas(name, compute)
     }
 }

 impl<K> SyscallHandler<K> for ChessEngineKernelImpl<K::CallManager>
 where
     K: ChessEngineKernel
         + ActorOps
         + SendOps
         + UpgradeOps
         + IpldBlockOps
         + CryptoOps
         + DebugOps
         + EventOps
         + MessageOps
         + NetworkOps
         + RandomnessOps
         + SelfOps,
 {
     fn link_syscalls(linker: &mut Linker<K>) -> anyhow::Result<()> {
         DefaultKernel::<K::CallManager>::link_syscalls(linker)?;

         linker.link_syscall("chess_engine_kernel", "chess_engine_syscall", chess_engine_syscall)?;

         Ok(())
     }
 }

 pub fn chess_engine_syscall(context: fvm::syscalls::Context<'_, impl ChessEngineKernel>) -> Result<u64> {
     context.kernel.chess_engine_syscall()
 }