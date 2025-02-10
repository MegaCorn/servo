/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

 use js::jsapi::{JSContext, JSString};

pub unsafe fn jsstr_to_string(cx: *mut JSContext, jsstr: *mut JSString) -> String {
    String::new()
}
