/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use js::jsapi::JSContext;

pub unsafe fn throw_type_error(cx: *mut JSContext, error: &str) {}

pub unsafe fn throw_range_error(cx: *mut JSContext, error: &str) {}