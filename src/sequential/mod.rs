// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

pub mod encryptor;
pub mod large_encryptor;
pub mod medium_encryptor;
pub mod small_encryptor;
pub mod utils;

pub use super::{COMPRESSION_QUALITY, MAX_CHUNK_SIZE, MAX_FILE_SIZE, MIN_CHUNK_SIZE,
                SelfEncryptionError, Storage, StorageError};
use encryption::{IV_SIZE, KEY_SIZE};
use rust_sodium::crypto::hash::sha256;

pub const HASH_SIZE: usize = sha256::DIGESTBYTES;
pub const PAD_SIZE: usize = (HASH_SIZE * 3) - KEY_SIZE - IV_SIZE;

pub struct Pad(pub [u8; PAD_SIZE]);
