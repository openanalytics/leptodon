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
#[derive(Default)]
pub enum GenericTheme {
    #[default]
    Brand,
    Secondary,
    Transparent,
    Danger,
    Success,
    Warning,
}

impl GenericTheme {
    // Color theme
    pub fn base_class(&self) -> &'static str {
        match self {
            GenericTheme::Brand => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
            GenericTheme::Secondary => {
                "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
            }
            GenericTheme::Transparent => "dark:text-white",
            GenericTheme::Danger => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
            GenericTheme::Success => {
                "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300"
            }
            GenericTheme::Warning => {
                "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300"
            }
        }
    }

    pub fn stroke_class(&self) -> &'static str {
        match self {
            GenericTheme::Brand => "!text-blue-800 dark:!text-blue-300",
            GenericTheme::Secondary => "!text-gray-800 dark:!text-gray-300",
            GenericTheme::Transparent => "",
            GenericTheme::Danger => "!text-red-800 dark:!text-red-300",
            GenericTheme::Success => "!text-green-800 dark:!text-green-300",
            GenericTheme::Warning => "!text-yellow-800 dark:!text-yellow-300",
        }
    }

    // Border and their colors
    pub fn border_class(&self) -> &'static str {
        match self {
            GenericTheme::Brand => "border border-blue-400",
            GenericTheme::Secondary => "border border-gray-400",
            GenericTheme::Transparent => "border border-gray-400",
            GenericTheme::Danger => "border border-red-400",
            GenericTheme::Success => "border border-green-400",
            GenericTheme::Warning => "border border-yellow-400",
        }
    }
}
