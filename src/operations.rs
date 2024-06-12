// Copyright 2024 Gustavo Mantovani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// src/operations.rs

use crate::core::change_wallpaper;
use crate::models::DwOperationExecuionResult;
use std::path::Path;

pub fn set_wallpaper(path: &String) -> DwOperationExecuionResult {
    // Chama a função change_wallpaper e captura o resultado
    match change_wallpaper(Path::new(path)) {
        Ok(_) => DwOperationExecuionResult {
            success: true,
            exit_code: 0,
            message: None,
        },
        Err(err) => {
            DwOperationExecuionResult {
                success: false,
                exit_code: 1,
                message: Some(err.to_string()), // Converte o erro para String
            }
        }
    }
}
