#!/usr/bin/env bash
## Testing all code directories in the repostitory using cargo test.

for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo test)
done

echo "Tests complete."