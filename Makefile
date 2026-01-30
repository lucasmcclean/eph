.PHONY: setup-hooks fmt sort all

# Setup Git hooks to use the committable .githooks directory
setup-hooks:
	@mkdir -p .git/hooks
	@echo '#!/bin/sh' > .git/hooks/pre-commit
	@echo 'HOOK="$$(git rev-parse --show-toplevel)/.githooks/pre-commit"' >> .git/hooks/pre-commit
	@echo 'if [ -x "$$HOOK" ]; then' >> .git/hooks/pre-commit
	@echo '    $$HOOK || { echo "ERROR: Pre-commit hook failed! Aborting commit." >&2; exit 1; }' >> .git/hooks/pre-commit
	@echo 'else' >> .git/hooks/pre-commit
	@echo '    echo "WARNING: Pre-commit hook not found or not executable: $$HOOK" >&2' >> .git/hooks/pre-commit
	@echo '    exit 1' >> .git/hooks/pre-commit
	@echo 'fi' >> .git/hooks/pre-commit
	@chmod +x .githooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Git pre-commit hook installed from .githooks/"

# Format all code
fmt:
	@cargo fmt --all
	@cargo sort-derives

# Check if all code is formatted
check:
	@cargo fmt --all --check
	@cargo sort-derives --check
	@cargo clippy
