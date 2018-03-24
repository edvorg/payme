#! /bin/bash

if [[ "${1}" == "test" ]] ; then
    RUST_BACKTRACE=1 cargo watch -x check -x test -i 'web-app/*'  -i 'pdf/*'
else
    RUST_BACKTRACE=1 cargo watch -x check -x test -x run -i 'web-app/*'  -i 'pdf/*'
fi
