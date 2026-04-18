<!--
 Copyright (C) 2026 wgrav
 
 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU Affero General Public License as
 published by the Free Software Foundation, either version 3 of the
 License, or (at your option) any later version.
 
 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU Affero General Public License for more details.
 
 You should have received a copy of the GNU Affero General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

# OpenCIQ
An open source ConnectIQ toolchain implementation.

# Roadmap/TODO:
> [!WARNING]
> This project is in the very early ages of its existence, **and is quite literally unusable at the time.** Features and roadmap are subject to change. Expect a lot of pushes to main. 

## Repo
- [x] Git init
- [ ] Tests, linting, Miri actions
- [ ] Nightly builds
- [ ] Documentation builder 
- [x] Ban AI with an AGENTS.md file
- [x] Megaban Claude
- [ ] Cargo cult in trunk.io/SonarQube/Codecov
- [ ] Finish this TODO list (so meta!)
- [ ] *Get a C&D from Garmin*
## Interpreter 
- [ ] **Tokenization/lexing**
- [ ] Repl for debugging
- [ ] AST generation
- [ ] Expression evaluation
- [ ] State
- [ ] Control flow
- [ ] Classes, functions, etc.
- [ ] More TODO items coming soon
## Compiler
- [ ] Finish the interpreter to use as a base
- [ ] Optimization passes
- [ ] Packaging
- [ ] More TODO items coming soon
## Bytecode VM
- [ ] Finish the compiler
- [ ] Decide if JIT is worth it (Cranelift)
- [ ] More TODO items coming soon
## Build System
- [ ] Multi file linking
- [ ] Project TOML file parsing
- [ ] Whole project compilation
- [ ] Test system
- [ ] Lock files
- [ ] Package manager
-# Note: The package manager will likely not have a centralized repository. 
## LSP
- [ ] More info coming soon 
## MCP
- [ ] Figure out if there should even be an MCP
## Debugger
- [ ] Decide between GDB server or internal debugger system
- [ ] More TODO items coming soon
## Type Enforcement
- [ ] Plan out a superset of Monkey C that is statically typed
- [ ] More TODO items coming soon
## Simulator
- [ ] Decide on what to use to show windows and content
- [ ] Figure out how to handle device types
- [ ] More TODO items coming soon
## Web UI:
- [ ] More TODO items coming soon.
-# (This is why the project uses the AGPL license instead of the standard GPL one.) 

**Key: Bolded items are actively being worked on, italicized items are up next.**

# AI Policy
AI is banned. Check the AGENTS.md file.

# Design Philosophy
More info coming soon

# Legal
- This project is not affiliated to Garmin in any way, shape, or form.
- It is based on official documentation and publicly reverse engineering efforts.
- No source code from Garmin's official toolchain is used.
- This project is licensed under the [GNU Affero General Public License](https://www.gnu.org/licenses/agpl-3.0.en.html) license. (SPDX: [AGPL-3.0-or-later](https://spdx.org/licenses/AGPL-3.0-or-later.html))
- **All contributors are required to sign off of commits to indicate they agree with the Developer Certificate of Origin.**