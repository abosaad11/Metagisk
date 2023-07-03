#![feature(format_args_nl)]
#![feature(btree_drain_filter)]

pub use base;
use cpio::cpio_commands;
use patch::{hexpatch, patch_encryption, patch_verity};
use payload::extract_boot_from_payload;
use sha::{get_sha, sha1_hash, sha256_hash, SHA};

mod cpio;
mod patch;
mod payload;
// Suppress warnings in generated code
#[allow(warnings)]
mod proto;
mod ramdisk;
mod sha;
mod bootimg;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("compress.hpp");
        fn decompress(buf: &[u8], fd: i32) -> bool;
    }

    extern "Rust" {
        type SHA;
        fn get_sha(use_sha1: bool) -> Box<SHA>;
        fn update(self: &mut SHA, data: &[u8]);
        fn finalize_into(self: &mut SHA, out: &mut [u8]);
        fn output_size(self: &SHA) -> usize;
        fn sha1_hash(data: &[u8], out: &mut [u8]);
        fn sha256_hash(data: &[u8], out: &mut [u8]);

        fn hexpatch(file: &[u8], from: &[u8], to: &[u8]) -> bool;
        fn patch_encryption(buf: &mut [u8]) -> usize;
        fn patch_verity(buf: &mut [u8]) -> usize;
    }

    #[namespace = "rust"]
    extern "Rust" {
        unsafe fn extract_boot_from_payload(
            partition: *const c_char,
            in_path: *const c_char,
            out_path: *const c_char,
        ) -> bool;

        unsafe fn cpio_commands(argc: i32, argv: *const *const c_char) -> bool;
    }
}
