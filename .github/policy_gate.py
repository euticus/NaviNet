import os, sys, json, subprocess

def get_repo():
    """Get owner/repo from GH_REPO env var or infer from origin remote."""
    gh_repo = os.environ.get("GH_REPO")
    if gh_repo:
        return gh_repo
    # Infer from git remote origin
    try:
        remote_url = subprocess.check_output(
            ["git", "remote", "get-url", "origin"], text=True
        ).strip()
        # Parse https://github.com/owner/repo.git or git@github.com:owner/repo.git
        if "github.com" in remote_url:
            if remote_url.startswith("https://"):
                # https://github.com/owner/repo.git
                parts = remote_url.replace("https://github.com/", "").replace(".git", "")
            else:
                # git@github.com:owner/repo.git
                parts = remote_url.split(":")[-1].replace(".git", "")
            return parts
    except Exception as e:
        print(f"Error inferring repo from origin: {e}")
    return None

def get_changed_files(pr_number, repo):
    cmd = f"gh api repos/{repo}/pulls/{pr_number}/files --paginate -q '.[].filename'"
    out = subprocess.check_output(cmd, shell=True, text=True)
    return [l.strip() for l in out.splitlines() if l.strip()]

def get_additions(pr_number, repo):
    cmd = f"gh api repos/{repo}/pulls/{pr_number} -q .additions"
    out = subprocess.check_output(cmd, shell=True, text=True)
    return int(out.strip())

def main():
    if len(sys.argv) < 3:
        print("usage: policy_gate.py <pr_number> <policy.json>"); sys.exit(1)
    pr = sys.argv[1]
    policy = json.load(open(sys.argv[2]))

    # Get repo from env or infer from origin
    repo = get_repo()
    if not repo:
        print("Error: Could not determine repo. Set GH_REPO env var."); sys.exit(1)

    files = get_changed_files(pr, repo)
    add = get_additions(pr, repo)

    deny_hits = [f for f in files for d in policy["denylist"] if __import__("fnmatch").fnmatch.fnmatch(f, d)]
    if deny_hits:
        print("Denied paths touched:", deny_hits); sys.exit(2)

    for f in files:
        if not any(__import__("fnmatch").fnmatch.fnmatch(f, a) for a in policy["allowlist"]):
            print("Path not allowlisted:", f); sys.exit(3)

    # Allow MAX_ADDITIONS env var to override policy.json (default 20000)
    max_additions = int(os.environ.get("MAX_ADDITIONS", "20000"))
    if add > max_additions:
        print(f"Diff too large: {add} > {max_additions}"); sys.exit(4)

    print("Policy OK")

if __name__ == "__main__":
    main()
