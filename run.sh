
# Uncomment the service that you wanna run

alacritty -e fish -c 'cd frontend/ && pnpm run dev; exec fish' &
# alacritty -e fish -c 'cd mailer/ && pnpm run dev; exec fish' &
# alacritty -e fish -c 'cd dashboard/ && cargo watch -c -w src -x run; exec fish' &

cd authentication/ && cargo watch -c -w src -x run
