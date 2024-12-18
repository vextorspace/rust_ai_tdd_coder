#!/bin/sh

echo "Find changed files..."

CHANGED_FILES=`git status -s | awk '{print $2}' | grep '\.rs$' | grep '^src/'| grep -v 'mod\.rs$' | grep -v 'lib\.rs$'`

echo "Changed files:"
for file in $CHANGED_FILES; do
    echo $file
done

cargo test --features unit_tests

TEST_RESULT=$?

CHANGED_FILES=`git status -s | awk '{print $2}'`
if [ -n "$CHANGED_FILES" ]; then
    # Check if tests failed
    if [ $TEST_RESULT -ne 0 ]; then
        echo "////////// Tests failed! Removing changes... //////////"
        git reset --hard HEAD
    else
        echo "++++++ Tests passed! Committing ++++++"
        git add .
        COMMIT_MESSAGE=$(ai_commit_message)
        echo "Commit message: $COMMIT_MESSAGE"
        git commit -m "$COMMIT_MESSAGE"
    fi
fi

echo "Running gradle tests..."
cargo test --test acceptance_tests

exit 0
