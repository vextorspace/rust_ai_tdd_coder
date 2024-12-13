#!/bin/sh

echo "Find changed files..."

CHANGED_FILES=`git status -s | awk '{print $2}' | grep '\.rs$' | grep '^src/'| grep -v 'mod\.rs$' | grep -v 'lib\.rs$'`

echo "Changed files:"
for file in $CHANGED_FILES; do
    echo $file
done

cargo test calc
# Store the test result
TEST_RESULT=$?

if [ -z "$CHANGED_FILES" ]; then
    echo "----------- No changed files detected -----------"

    # Check if tests failed
    if [ $TEST_RESULT -ne 0 ]; then
        echo "////////// Tests failed! Removing changes... //////////"
        git reset --hard HEAD
    else
        echo "++++++ Tests passed! Committing ++++++"
        git add .
        COMMIT_MESSAGE=`ai_commit_message ./`
        echo "Commit message: $COMMIT_MESSAGE"
        git commit -m "$COMMIT_MESSAGE"
    fi
else
    if [ $TEST_RESULT -ne 0 ]; then
      FIRST_CHANGED_FILE=$(echo "$CHANGED_FILES" | head -n 1)

      echo "========= > Writing code < ========="

      OUTPUT=$(cargo test calc 2>&1)

      # Execute ai_coder
      ai_coder "$FIRST_CHANGED_FILE" "$OUTPUT"

      cargo test calc
      # Store the test result
      TEST_RESULT=$?
    fi
    # Check if tests failed
    if [ $TEST_RESULT -ne 0 ]; then
        echo "/////// Tests failed! Removing changes... ///////"
        git reset --hard HEAD
    else
        echo "++++++ Tests passed! Committing... ++++++"
        git add .
        COMMIT_MESSAGE=`ai_commit_message ./`
        git commit -m "$COMMIT_MESSAGE"
    fi
fi



echo "Running gradle tests..."
cargo test --test acceptance_tests

exit 0
