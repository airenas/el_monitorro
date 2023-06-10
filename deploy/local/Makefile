-include .env
##################################################
start: init/postgresql/init.sql
	docker compose up -d
.PHONY: start
##################################################
init/postgresql/init.sql: init/postgresql/init.sql.in .env
	cat init/postgresql/init.sql.in | envsubst > $@
psql/feeds_user:
	docker compose exec -e PGPASSWORD=$(DB_PASS) db psql -U feeds_user -h localhost -d el_monitorro
.PHONY: psql/feeds_user
##################################################
clean:
	docker compose down
.PHONY: clean
##################################################
.EXPORT_ALL_VARIABLES:	