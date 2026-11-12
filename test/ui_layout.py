#!/usr/bin/env python3
"""Layout regression test: the window chrome must not move.

Launches a debug intelliED on a scratch project through `edctl`
(`spec/automation.md`), then clicks every tool window stripe button and
switches every mode. After each step the main toolbar, the stripe buttons and
the status bar must sit exactly where they did at the start.

Run from the repo root after `cargo build -p ed -p control`:

    python3 test/ui_layout.py
"""

import json
import os
import re
import subprocess
import sys
import tempfile

EDCTL = os.path.join(os.path.dirname(__file__), "..", "target", "debug", "edctl")

# Chrome that must stay put: id pattern -> stable name.
CHROME = [
    (r"^main-toolbar$", "main-toolbar"),
    (r"^status-bar$", "status-bar"),
    (r"/search-everywhere$", "search-everywhere"),
    (r"/find-action$", "find-action"),
    (r"^ButtonLike/(Project Panel|GitPanel|Outline Panel|TerminalPanel)-\d+$", None),
    (r"^ButtonLike/(stripe-git|stripe-problems)$", None),
]


def edctl(*args, check=True):
    result = subprocess.run([EDCTL, *args], capture_output=True, text=True)
    if check and result.returncode != 0:
        raise RuntimeError(f"edctl {' '.join(args)}: {result.stderr.strip()}")
    return result.stdout


def chrome_bounds():
    """Bounds of every chrome element in the last frame, by stable name."""
    edctl("settle")
    bounds = {}
    for element in json.loads(edctl("tree", "--json"))["elements"]:
        for pattern, name in CHROME:
            match = re.search(pattern, element["id"])
            if match:
                key = name or match.group(1)
                b = element["bounds"]
                bounds[key] = (b["x"], b["y"], b["width"], b["height"])
    return bounds


def scratch_project():
    root = tempfile.mkdtemp(prefix="intellied-ui-test-")
    os.makedirs(os.path.join(root, "src"))
    with open(os.path.join(root, "src", "main.rs"), "w") as file:
        file.write('fn main() {\n    println!("hi");\n}\n')
    subprocess.run(["git", "init", "-q", root], check=True)
    subprocess.run(["git", "-C", root, "add", "-A"], check=True)
    subprocess.run(
        ["git", "-C", root, "-c", "user.name=test", "-c", "user.email=test@example.com", "commit", "-qm", "init"],
        check=True,
    )
    return root


def main():
    edctl("launch", "--fresh", scratch_project())
    failures = []
    try:
        baseline = chrome_bounds()
        expected = {"main-toolbar", "status-bar", "Project Panel", "GitPanel", "Outline Panel", "TerminalPanel",
                    "stripe-git", "stripe-problems"}
        missing = expected - baseline.keys()
        if missing:
            failures.append(f"start: missing chrome {sorted(missing)}")

        steps = [(f"click {name}", ("click", "--text", text)) for name, text in [
            ("Project", "Project"), ("Commit", "Commit"), ("Structure", "Structure"),
            ("Git", "Git"), ("Terminal", "Terminal"), ("Problems", "Problems"),
        ]]
        steps += [
            ("Git mode key", ("keys", "cmd-9")),
            ("Terminal mode key", ("keys", "alt-shift-f12")),
            ("zoom", ("action", "workspace::ToggleZoom")),
            ("tool window names", ("action", "workspace::ToggleToolWindowNames")),
        ]
        for name, command in steps:
            # Each step runs twice: on, then off again.
            for phase in ("on", "off"):
                edctl(*command, "--nth", "0", check=False) if command[0] == "click" else edctl(*command, check=False)
                now = chrome_bounds()
                if name == "tool window names" and phase == "on":
                    continue  # the stripes are meant to narrow; "off" must restore them
                for key, bounds in baseline.items():
                    if now.get(key) != bounds:
                        failures.append(f"{name} ({phase}): {key} moved {bounds} -> {now.get(key)}")
    finally:
        edctl("quit", check=False)

    for failure in failures:
        print("FAIL", failure)
    print("ui_layout:", "FAILED" if failures else "ok")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
