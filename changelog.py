#!/usr/bin/python3

# This Script is to be used when incrementing the version for release
# It will automatically look through the commits, build a changelog entry,
# and then write it to the changelog

import json
from subprocess import run
from pathlib import Path

def command(cmd: str) -> str:
	"""Run the command, capture output and return text stdout."""
	return run(cmd.split(), capture_output=True, text=True).stdout


def generate_change_map(version: str) -> dict[str, list[str]]:
	"""Generate and sort the change map."""
	change_map = {
		"Changes": [],
		"Bug Fixes": [],
	}

	git_output = command(f"git log v{version}..HEAD --oneline --no-decorate").splitlines()

	for change in git_output:
		commit_id, commit_type, message = change.split(maxsplit=2)

		if commit_type in ("fix:"):
			change_map["Bug Fixes"].append(
				# Make sure the first letter is capitalized for proper entry.
				message.capitalize()
			)

		if commit_type in ("change:"):
			change_map["Changes"].append(message)
	return change_map


def generate_log_entry(version: str):
	"""Generate change log entry."""
	log_entry = f"rust-apt ({version}) unstable; urgency=medium\n\n"

	for header, messages in generate_change_map(version).items():
		# No need to add a header if there aren't any
		if not messages:
			continue

		log_entry += f"  [ {header} ]\n"
		for message in messages:
			log_entry += f"  * {message}\n"

	log_entry += f"\n -- Blake Lee <blake@volian.org>  {command('date -R')}\n"
	return log_entry

def main() -> None:
	"""Generate the changelog entry and write it to the changelog."""
	change_log = Path("./changelog")

	cargo_metadata = json.loads(
		command("cargo metadata --no-deps --format-version=1")
	)

	data = generate_log_entry(
		cargo_metadata["packages"][0]["version"]
	)
	# If the changelog already exists, append its text to our entry
	if change_log.exists():
		data += change_log.read_text()

	change_log.write_text(data)

if __name__ == "__main__":
	main()
