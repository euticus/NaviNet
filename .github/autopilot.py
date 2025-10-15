import os, json, subprocess, sys, yaml

STATE = ".augment/state.json"
PLAYBOOK = ".augment/playbook.yaml"
NEXT = "NEXT_TASK.md"

def sh(cmd, check=False):
    print("+", cmd, flush=True)
    p = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if check and p.returncode != 0:
        print(p.stdout); print(p.stderr, file=sys.stderr); sys.exit(p.returncode)
    return p

def read_yaml(p): return yaml.safe_load(open(p))
def read_json(p): return json.load(open(p))
def write_json(p, obj): open(p, "w").write(json.dumps(obj, indent=2))

def ci_green():
    # Consider green if repo builds & tests pass for Rust workspace (and JS if present).
    if os.path.exists("Cargo.toml"):
        if sh("rustup show").returncode != 0: return False
        if sh("cargo fmt --all --check").returncode != 0: return False
        if sh("cargo clippy --all-targets --all-features -D warnings").returncode != 0: return False
        if sh("cargo test --workspace --all-features --quiet").returncode != 0: return False
    if os.path.exists("package.json"):
        if sh("npm ci").returncode != 0: return False
        if sh("npm run build --if-present").returncode != 0: return False
        if sh("npm test --silent --if-present").returncode != 0: return False
    return True

def write_next(task):
    with open(NEXT, "w") as f:
        f.write(f"# Augment Task: {task['id']} – {task['title']}\n\n")
        f.write(task["prompt"].strip() + "\n")
        f.write("\n---\n\n")
        f.write("**Bot instructions**:\n")
        f.write("- Create a PR titled `[AUTO] {id}` for your changes.\n".format(id=task["id"]))
        f.write("- Keep diffs focused; update README where applicable.\n")
        f.write("- Ensure all CI checks pass.\n")
    sh(f'git add {NEXT}')
    sh(f'git commit -m "autopilot: NEXT_TASK -> {task["id"]}" || true')

def main():
    pb, st = read_yaml(PLAYBOOK), read_json(STATE)
    tasks = pb["tasks"]

    if st["current_task"] is None:
        task = tasks[0]
    else:
        ids = [t["id"] for t in tasks]
        idx = ids.index(st["current_task"])
        if idx >= len(tasks) - 1:
            print("Autopilot: all tasks complete."); return
        task = tasks[idx + 1]

    if not ci_green():
        print("Autopilot: CI not green; not advancing."); return

    st["current_task"] = task["id"]
    write_json(STATE, st)
    sh(f'git add {STATE}')
    write_next(task)
    # Push back to current branch
    ref = os.environ.get("GITHUB_REF_NAME", "main")
    sh(f'git push origin HEAD:{ref}', check=True)

if __name__ == "__main__":
    main()
