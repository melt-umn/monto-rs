all: check doc build build-release
build:
	cargo build --all
build-release:
	cargo build --all --release
check:
	cargo check --all
doc:
	cargo doc --all
watch TARGET="all":
	watchexec -cr "just {{TARGET}}"
