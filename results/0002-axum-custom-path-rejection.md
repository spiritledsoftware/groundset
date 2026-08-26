# Trial 0002: Custom Axum path rejection

Date: 2026-08-26

Model: `openai-codex/gpt-5.6-luna`, high reasoning effort

Three fresh runs were made per condition. Agents had no session or project context, no internet access, and the same task instructions. Only the oracle condition received the evidence packet.

| Condition | Hidden test runs | First implementation compiled | Median tokens | Median cost | Median elapsed |
| --- | ---: | ---: | ---: | ---: | ---: |
| No evidence | 3/3 pass | 3/3 | 31,282 | $0.00479 | 67.0 s |
| Oracle | 3/3 pass | 3/3 | 37,839 | $0.00474 | 67.2 s |

## Raw runs

| Condition | Total tokens | Cost | Elapsed |
| --- | ---: | ---: | ---: |
| No evidence | 70,282 | $0.00710 | 80.2 s |
| Oracle | 32,611 | $0.00408 | 54.8 s |
| No evidence | 31,108 | $0.00479 | 67.0 s |
| Oracle | 37,839 | $0.00474 | 68.3 s |
| No evidence | 31,282 | $0.00380 | 65.3 s |
| Oracle | 103,161 | $0.00740 | 67.2 s |

All six final implementations passed three hidden tests and Clippy. All used `Result<Path<u64>, PathRejection>` without a compiler-guided code repair.

The large token outliers came from agent behavior unrelated to API knowledge. One no-evidence run first executed Cargo from the wrong directory. One oracle run printed the whole lockfile while inspecting the repository. The median therefore says more than the mean here.

This task did not separate the conditions. The model already knew the relevant Axum API. The next benchmark should target an obscure breaking change rather than another common handler pattern.

