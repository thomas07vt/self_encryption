// Copyright 2015 MaidSafe.net limited.
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

// TODO(dirvine) Look at aessafe 256X8 cbc it should be very much faster  :01/03/2015

use rust_sodium::crypto::secretbox::{self, KEYBYTES, NONCEBYTES};

pub use rust_sodium::crypto::secretbox::Key;
pub use rust_sodium::crypto::secretbox::Nonce as Iv;
pub type DecryptionError = ();

pub const KEY_SIZE: usize = KEYBYTES;
pub const IV_SIZE: usize = NONCEBYTES;

pub fn encrypt(data: &[u8], key: &Key, iv: &Iv) -> Vec<u8> {
    secretbox::seal(data, iv, key)
}

pub fn decrypt(encrypted_data: &[u8], key: &Key, iv: &Iv) -> Result<Vec<u8>, DecryptionError> {
    secretbox::open(encrypted_data, iv, key)
}
