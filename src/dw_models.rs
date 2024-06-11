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

// src/dw_models.rs

use std::process::Output;

#[derive(Debug)]
pub struct DwExecuionResult {
    pub sucess: bool,
    pub exit_code: u8,
    pub message: String,
    pub sys_commando_execution_output: Option<Output>,
}
