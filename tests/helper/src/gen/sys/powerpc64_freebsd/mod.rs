// This file is @generated by portable-atomic-internal-codegen
// (gen function at tools/codegen/src/ffi.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
mod sys_auxv;
pub use sys_auxv::elf_aux_info;
mod sys_syscall;
pub use sys_syscall::SYS_getpid;
pub use sys_syscall::SYS___sysctl;
mod sys_sysctl;
pub use sys_sysctl::CTL_KERN;
pub use sys_sysctl::KERN_PROC;
pub use sys_sysctl::KERN_PROC_AUXV;
pub use sys_sysctl::sysctl;
mod sys_elf_common;
pub use sys_elf_common::AT_HWCAP;
pub use sys_elf_common::AT_HWCAP2;
pub use sys_elf_common::AT_COUNT;
mod unistd;
pub use unistd::getpid;
mod machine_elf;
pub use machine_elf::Elf64_Auxinfo;
mod machine_cpu;
pub use machine_cpu::PPC_FEATURE_32;
pub use machine_cpu::PPC_FEATURE_64;
pub use machine_cpu::PPC_FEATURE_601_INSTR;
pub use machine_cpu::PPC_FEATURE_HAS_ALTIVEC;
pub use machine_cpu::PPC_FEATURE_HAS_FPU;
pub use machine_cpu::PPC_FEATURE_HAS_MMU;
pub use machine_cpu::PPC_FEATURE_UNIFIED_CACHE;
pub use machine_cpu::PPC_FEATURE_HAS_SPE;
pub use machine_cpu::PPC_FEATURE_HAS_EFP_SINGLE;
pub use machine_cpu::PPC_FEATURE_HAS_EFP_DOUBLE;
pub use machine_cpu::PPC_FEATURE_NO_TB;
pub use machine_cpu::PPC_FEATURE_POWER4;
pub use machine_cpu::PPC_FEATURE_POWER5;
pub use machine_cpu::PPC_FEATURE_POWER5_PLUS;
pub use machine_cpu::PPC_FEATURE_CELL;
pub use machine_cpu::PPC_FEATURE_BOOKE;
pub use machine_cpu::PPC_FEATURE_SMT;
pub use machine_cpu::PPC_FEATURE_ICACHE_SNOOP;
pub use machine_cpu::PPC_FEATURE_ARCH_2_05;
pub use machine_cpu::PPC_FEATURE_HAS_DFP;
pub use machine_cpu::PPC_FEATURE_POWER6_EXT;
pub use machine_cpu::PPC_FEATURE_ARCH_2_06;
pub use machine_cpu::PPC_FEATURE_HAS_VSX;
pub use machine_cpu::PPC_FEATURE_TRUE_LE;
pub use machine_cpu::PPC_FEATURE_PPC_LE;
pub use machine_cpu::PPC_FEATURE2_ARCH_2_07;
pub use machine_cpu::PPC_FEATURE2_HTM;
pub use machine_cpu::PPC_FEATURE2_DSCR;
pub use machine_cpu::PPC_FEATURE2_EBB;
pub use machine_cpu::PPC_FEATURE2_ISEL;
pub use machine_cpu::PPC_FEATURE2_TAR;
pub use machine_cpu::PPC_FEATURE2_HAS_VEC_CRYPTO;
pub use machine_cpu::PPC_FEATURE2_HTM_NOSC;
pub use machine_cpu::PPC_FEATURE2_ARCH_3_00;
pub use machine_cpu::PPC_FEATURE2_HAS_IEEE128;
pub use machine_cpu::PPC_FEATURE2_DARN;
pub use machine_cpu::PPC_FEATURE2_SCV;
pub use machine_cpu::PPC_FEATURE2_HTM_NOSUSPEND;
pub use machine_cpu::PPC_FEATURE_BITMASK;
pub use machine_cpu::PPC_FEATURE2_BITMASK;
pub type c_char = u8;
