# Trial 0003: Axum WebSocket message payloads

Date: 2026-08-26

Model: `openai-codex/gpt-5.6-luna`, high reasoning effort

Each condition used a fresh copy of the starter repository. Agents had no session or project context, no internet access, and the same task instructions. Only the oracle condition received the evidence packet.

| Condition | Hidden tests | First implementation compiled | Total tokens | Cost | Elapsed |
| --- | --- | --- | ---: | ---: | ---: |
| No evidence | 5/5 pass | Yes | 28,240 | $0.00340 | 42.9 s |
| Oracle | 5/5 pass | Yes | 23,751 | $0.00343 | 36.0 s |

Both runs used the Axum 0.8 `Message::text` constructor and preserved the new `Bytes` payloads without a repair cycle. The oracle run used fewer tokens and less time, but one pair cannot distinguish an evidence effect from ordinary run variance.

This model already knows the tested Axum 0.8 breaking changes. Adding more common Axum tasks is unlikely to test knowledge externalization. The next experiment should use a less knowledgeable model or an obscure API whose current version is unlikely to be present in training.

