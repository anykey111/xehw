## Archived: 2026-11-02

* An 'Agent Auto-Fix' button in the status bar triggers an agent request with the current error and code context when a VM error occurs.
* The `AgentSystem` implements `spawn_planning_request` to prompt an LLM to break down a specific goal into a list of tasks, utilizing the current code as context.
* The user prefers to keep the entire codebase in Rust, avoiding the use of other languages (e.g., replacing Python scripts with Rust binaries).
* The project uses the `syntect` crate (v5.3.0) for syntax highlighting, implemented in `src/layouter.rs`.
* Run the broadcast server using the command: `cargo run --release --bin server`.
* The project uses `ehttp` (v0.5.0) which requires manual JSON serialization (e.g., using `serde_json`) as it lacks the `Request::json` helper method.
* The application implements a simple RAG system via `TemplateApp::get_relevant_docs`, which searches `self.help.words` for keywords matching the agent query to provide context.
* Agent configuration in `src/app.rs` allows dynamic specification of the Model and Base URL via the UI, defaulting to `llama3` and `http://localhost:11434`.
* The 'File Explorer' feature is guarded by `#[cfg(not(target_arch = "wasm32"))]` and is not available in WebAssembly builds.
* When implementing undo functionality for agent actions, `snapshot()` must be called *before* applying any code modifications.
* Agent interactions in `AgentSystem::poll` collect requests into a vector to be spawned after the iteration loop to avoid mutable borrow conflicts.
* The project utilizes `egui` (v0.30) and `eframe` (v0.30) for the GUI and `wasm-bindgen` for web targets.
* The `AgentEvent` enum in `src/agent.rs` includes a `PlanResponse` variant to handle AI-generated task lists.
* The Workspace feature (defined in `src/workspace.rs`) persists named sessions containing both the current code (`live_code`) and the agent task list (`tasks`).
* `AgentEvent` variants (e.g., `LlmResponse`, `ChatResponse`) include a workspace name (String) to ensure events are processed only within their originating workspace (Workspace Isolation).
* Use `id_salt` instead of the deprecated `id_source` for `egui` widgets (e.g., `ScrollArea`) to ensure compatibility with `egui` v0.30+.
* The `Agent` struct in `src/agent.rs` includes `chat_history` (a `Vec<ChatMessage>`) and `chat_input` fields to support interactive chat sessions alongside task execution.
* A `TODO.md` file records the project roadmap, tracking completed tasks and including a 'Last Will and Testament' section with architectural notes for future developers.
* Avoid using `unsafe` blocks with static mutable variables for temporary UI state; instead, store state fields (like `new_workspace_name`) directly in the main application struct.
* The project uses the `fuzzy-matcher` crate to implement fuzzy search functionality within the application.
* The `AgentSystem::spawn_llm_request` method is public (`pub`) to facilitate direct LLM requests from the main application logic, such as for the error auto-fix feature.
* The `ui_mini_status` method in `src/app.rs` accepts `&mut self` to enable interactive elements like the 'Fix with Agent' button that modify application state.
* The Workspace feature supports 'Forking', which creates a named copy of the current workspace (appending `-copy`) with duplicated code and tasks.
* Asynchronous chat responses from agents are handled using the `AgentEvent::ChatResponse` enum variant.
* The language reference documentation is stored in `assets/help.xeh` and loaded via `include_str!`, whereas the GUI hotkey help list is defined programmatically in `src/app.rs`.
* A native-only File Explorer feature is implemented in `src/file_explorer.rs`, utilizing `std::fs` and `egui::CollapsingHeader` for directory traversal.
* Agent-generated code changes are intercepted and stored in `pending_reviews` (a `VecDeque<(Uuid, String)>`) within `TemplateApp` for user approval instead of being applied immediately.
* The project uses the `similar` crate to calculate text differences, which are rendered in the Code Review UI with green (insert) and red (delete) highlights.
* The `ehttp` library is used for HTTP requests and does not support cancelling in-flight requests (fire-and-forget).
* The `Task` struct in `src/agent.rs` includes a `context` field (String) used to pass relevant documentation (RAG) to the LLM.
* Switching workspaces explicitly clears `pending_reviews` and resets agent statuses to ensure complete isolation between coding sessions.
* Agent state (the `agents` map) is persisted to `eframe` storage using the key "agents".
* The 'ToDo List' UI in `src/app.rs` includes a 'Generate Plan' feature, utilizing a `planning_goal` state field to capture user input for AI task generation.
* The `AgentSystem`'s `ui_tasks` method includes a visual progress bar (`egui::ProgressBar`) that tracks the completion percentage of agent tasks.
* The `AgentConfig` struct includes a `system_prompt` field, allowing users to define specific roles or instructions which are prepended to the chat history during LLM requests.
* Run the main application using the command: `cargo run --release`.
* The collaborative backend is a Rust binary (`src/bin/server.rs`) dependent on the `server` feature (enabled by default). It uses `tokio`, `tokio-tungstenite`, `futures-util`, and `tokio-stream`.
* The Command Palette is implemented in `src/palette.rs` and is triggered by `Cmd+Shift+P` on macOS or `Ctrl+Shift+P` on other platforms.
