# PROC-VIEW

A lightweight Linux process manager built with Rust and egui.

🚀 Features

* **Process Listing:** Read and display all active system processes in real-time.
* **Deep Stat Inspection:** View full details (PID, State, RSS RAM, Virtual Memory) by parsing `/proc/[PID]/stat`.
* **Process Killer:** Terminate any selected process instantly using secure PID-based `SIGKILL`.

⚙️ How it works

1. Scans the Linux `/proc` directory for active numeric process IDs.
2. Extracts clean process names from the `comm` file.
3. Fixes state layout into a stable snapshot so rows don't jump around.
4. Spawns an interactive dropdown menu for every process to inspect or kill it.

🛠 Tech Stack

* **Language:** Rust
* **GUI Framework:** egui & eframe
* **OS Interface:** Linux `/proc` filesystem & `std::process::Command`

📦 Run

```bash
cargo run
