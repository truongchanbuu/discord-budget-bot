#!/usr/bin/env bash
set -euo pipefail
fundguard_pg_bin="${FUNDGUARD_TEST_PG_BIN:-$(pg_config --bindir)}"
for program in postgres initdb pg_ctl; do
    if [[ ! -x "$fundguard_pg_bin/$program" ]]; then
        echo "PostgreSQL server tools required; set FUNDGUARD_TEST_PG_BIN to their directory." >&2
        exit 1
    fi
done
fundguard_test_dir=$(mktemp -d /tmp/fundguard-pg.XXXXXX)
chmod 700 "$fundguard_test_dir"
cleanup() {
    "$fundguard_pg_bin/pg_ctl" -D "$fundguard_test_dir/data" -m immediate stop >/dev/null 2>&1 || true
    rm -rf -- "$fundguard_test_dir"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
mkdir "$fundguard_test_dir/socket"
"$fundguard_pg_bin/initdb" -D "$fundguard_test_dir/data" --username=fundguard_test --auth-local=trust --auth-host=reject --no-locale --encoding=UTF8 >/dev/null
"$fundguard_pg_bin/pg_ctl" -D "$fundguard_test_dir/data" -l "$fundguard_test_dir/server.log" -o "-F -k $fundguard_test_dir/socket -p 55438 -c listen_addresses=''" -w start >/dev/null
createdb -h "$fundguard_test_dir/socket" -p 55438 -U fundguard_test fundguard_test
export FUNDGUARD_TEST_PRIVATE_CLUSTER=1
export FUNDGUARD_TEST_DATABASE_URL="postgresql://fundguard_test@localhost:55438/fundguard_test?host=$fundguard_test_dir/socket"
cargo test --locked --lib features::ledger::infrastructure -- --ignored --test-threads=1
