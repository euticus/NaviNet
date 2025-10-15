import os, sys, json, subprocess

def get_changed_files(pr_number):
    cmd = f"gh api repos/{{owner}}/{{repo}}/pulls/{pr_number}/files --paginate -q '.[].filename'"
    out = subprocess.check_output(cmd, shell=True, text=True)
    return [l.strip() for l in out.splitlines() if l.strip()]

def get_additions(pr_number):
    cmd = f"gh api repos/{{owner}}/{{repo}}/pulls/{pr_number} -q .additions"
    out = subprocess.check_output(cmd, shell=True, text=True)
    return int(out.strip())

def main():
    if len(sys.argv) < 3:
        print("usage: policy_gate.py <pr_number> <policy.json>"); sys.exit(1)
    pr = sys.argv[1]
    policy = json.load(open(sys.argv[2]))
    files = get_changed_files(pr)
    add = get_additions(pr)

    deny_hits = [f for f in files for d in policy["denylist"] if __import__("fnmatch").fnmatch.fnmatch(f, d)]
    if deny_hits:
        print("Denied paths touched:", deny_hits); sys.exit(2)

    for f in files:
        if not any(__import__("fnmatch").fnmatch.fnmatch(f, a) for a in policy["allowlist"]):
            print("Path not allowlisted:", f); sys.exit(3)

    if add > policy.get("max_additions", 999999):
        print(f"Diff too large: {add} > max"); sys.exit(4)

    print("Policy OK")

if __name__ == "__main__":
    main()
