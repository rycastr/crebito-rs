.EXPORT_ALL_VARIABLES:
DATABASE_URL = postgresql://postgres:postgres@localhost:5432/crebito

.PHONY: setup
setup:
	@cargo sqlx database create
	@cargo sqlx migrate run

destroy:
	@cargo sqlx database drop

prepare:
	@cargo sqlx prepare

run:
	@cargo run