// Copyright 2026 Geraldo Netto
// SPDX-License-Identifier: Apache-2.0

//! Read/write visibility ordering; fixed scratch is borrowed for this call only.
fn rank(name: &str) -> Option<u8> {
    match name {
        "T_PUBLIC" | "T_PUBLIC_SET" => Some(0),
        "T_PROTECTED" | "T_PROTECTED_SET" => Some(1),
        "T_PRIVATE" | "T_PRIVATE_SET" => Some(2),
        _ => None,
    }
}

pub(super) fn ordered(names: &[Option<&str>]) -> bool {
    let mut read = 0; // Omitted read visibility is public.
    let mut write = None;
    for &name in names.iter().flatten() {
        let Some(value) = rank(name) else { continue };
        if name.ends_with("_SET") {
            write = Some(value);
        } else {
            read = value;
        }
    }
    write.is_none_or(|value| value >= read)
}
