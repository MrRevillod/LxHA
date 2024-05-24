#!/bin/bash

# Navegar al directorio Frontend y ejecutar pnpm run dev en segundo plano
cd Frontend
pnpm run dev &
FRONTEND_PID=$!
cd ..

# Navegar al directorio mailer y ejecutar pnpm run dev en segundo plano
cd mailer
pnpm run dev &
MAILER_PID=$!
cd ..

# Navegar al directorio dashboard y ejecutar cargo run en segundo plano
cd dashboard
cargo run &
DASHBOARD_PID=$!
cd ..

# Navegar al directorio authentication y ejecutar cargo run
cd authentication
cargo run
AUTH_PID=$!

# Esperar a que todos los procesos en segundo plano finalicen
wait $FRONTEND_PID
wait $MAILER_PID
wait $DASHBOARD_PID
wait $AUTH_PID
echo "Frontend service started with PID $FRONTEND_PID"
echo "Mailer service started with PID $MAILER_PID"
echo "Dashboard service started with PID $DASHBOARD_PID"
echo "Authentication service started with PID $AUTH_PID"

