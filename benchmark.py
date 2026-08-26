#!/usr/bin/env python3

import argparse
import json
import shutil
import subprocess
import tempfile
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parent
TASKS = ROOT / "benchmark" / "tasks"
CONDITIONS = ("no-evidence", "oracle")


def task_path(name: str) -> Path:
    path = TASKS / name
    if not path.is_dir() or path.parent != TASKS:
        raise SystemExit(f"unknown task: {name}")
    return path


def prepare(task: str, condition: str, destination: Path) -> None:
    source = task_path(task)
    if destination.exists():
        raise SystemExit(f"destination already exists: {destination}")

    shutil.copytree(source / "starter", destination, ignore=shutil.ignore_patterns("target"))
    prompt = (source / "task.md").read_text()
    if condition == "oracle":
        prompt += "\n\n" + (source / "evidence" / "oracle.md").read_text()
    (destination / "PROMPT.md").write_text(prompt)
    print(destination)


def verify(task: str, candidate: Path) -> None:
    source = task_path(task)
    if not (candidate / "Cargo.toml").is_file():
        raise SystemExit(f"not a candidate Rust repository: {candidate}")

    commands = (
        ("cargo", "test", "--all-targets"),
        ("cargo", "clippy", "--all-targets", "--", "-D", "warnings"),
    )
    results = []

    with tempfile.TemporaryDirectory(prefix=f"{task}-") as directory:
        work = Path(directory) / "candidate"
        shutil.copytree(candidate, work, ignore=shutil.ignore_patterns("target"))
        shutil.copytree(source / "tests", work / "tests", dirs_exist_ok=True)

        for command in commands:
            started = time.monotonic()
            result = subprocess.run(command, cwd=work, text=True)
            results.append(
                {
                    "command": " ".join(command),
                    "exit_code": result.returncode,
                    "seconds": round(time.monotonic() - started, 3),
                }
            )
            if result.returncode:
                break

    report = {"task": task, "passed": all(r["exit_code"] == 0 for r in results), "checks": results}
    print(json.dumps(report, indent=2))
    raise SystemExit(0 if report["passed"] else 1)


def main() -> None:
    parser = argparse.ArgumentParser()
    commands = parser.add_subparsers(dest="command", required=True)

    prepare_parser = commands.add_parser("prepare")
    prepare_parser.add_argument("task")
    prepare_parser.add_argument("condition", choices=CONDITIONS)
    prepare_parser.add_argument("destination", type=Path)

    verify_parser = commands.add_parser("verify")
    verify_parser.add_argument("task")
    verify_parser.add_argument("candidate", type=Path)

    args = parser.parse_args()
    if args.command == "prepare":
        prepare(args.task, args.condition, args.destination)
    else:
        verify(args.task, args.candidate)


if __name__ == "__main__":
    main()
