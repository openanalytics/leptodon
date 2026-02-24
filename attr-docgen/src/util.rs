// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
/// Changes "hello" into hello
/// Does not change "hello or "hello
pub(crate) fn trim_surrounding_quotes(mut str: String) -> String {
    if str.ends_with("\"") && str.starts_with("\"") {
        // Assumes string literal is surrounded by " ";
        str.truncate(str.len() - 1);
        return str[1..].trim().to_string();
    }
    str
}
