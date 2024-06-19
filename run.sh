#!/bin/bash

alacritty -e fish -c 'cd frontend/ && pnpm run dev; exec fish' &
alacritty -e fish -c 'cd dashboard/ && cargo watch -c -w src -x run; exec fish' &
alacritty -e fish -c 'cd admin/ && cargo watch -c -w src -x run; exec fish' &
alacritty -e fish -c 'cd mailer-rs/ && cargo watch -c -w src -x run; exec fish' &
#alacritty -e fish -c 'cd monitoring/ && cargo watch -c -w src -x run; exec fish' &
#alacritty -e fish -c 'cd mailer/ && pnpm run dev; exec fish' &

cd authentication/ && cargo watch -c -w src -x run
