#!/bin/bash
# Check commit message follows conventional commits and title is ≤72 chars

title=$(head -n1 "$1")

# Check conventional commit format
if ! echo "$title" | grep -qE "^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\(.+\))?: .+"; then
    echo "❌ Commit title must follow conventional commits format:"
    echo "   type(scope): description"
    echo "   Types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert"
    exit 1
fi

# Check 72 char limit
if [ ${#title} -gt 72 ]; then
    echo "❌ Commit title is ${#title} chars (max 72)"
    exit 1
fi

exit 0
