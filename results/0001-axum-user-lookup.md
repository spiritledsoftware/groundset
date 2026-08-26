# Trial 0001: Axum user lookup

Date: 2026-08-26

Model: `openai-codex/gpt-5.6-luna`, high reasoning effort

Each condition used a fresh copy of the starter repository. The agents had no session or project context, no internet access, and the same task instructions. Only the oracle condition received the evidence packet.

| Condition | Hidden tests | Clippy | Attempts | Total tokens | Cost | Elapsed |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| No evidence | Pass | Pass | 2 | 55,991 | $0.00560 | 58.5 s |
| Oracle | Pass | Pass | 1 | 43,171 | $0.00397 | 43.5 s |

The no-evidence run first returned `Option<Json<User>>`, which Axum 0.8.9 rejected as a handler response. It repaired the code after compilation failed. The oracle run used `Result<Json<User>, StatusCode>` immediately. Both final implementations were equivalent and used the correct Axum 0.8 `/{id}` route syntax.

This task was too easy to separate the conditions by final correctness. In this single run, the evidence packet avoided one repair cycle. More runs and harder version-sensitive tasks are needed before drawing a conclusion.

