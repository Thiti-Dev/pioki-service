dev:
	cargo watch -x 'run'
install-diesel-cli:
	cargo install diesel_cli --no-default-features --features postgres
diesel-setup:
	diesel setup
create-migration:# -> make create-migration name=migration_name
	diesel migration generate ${name}
migration-run:
	diesel migration run
migration-redo:
	diesel migration redo
migration-rollback:
	diesel migration rollback

.PHONY: dev install-diesel-cli diesel-setup create-migration migration-redo migration-rollback