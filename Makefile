.DEFAULT_GOAL := default
SHELL := /bin/bash


.PHONY: default
default: | help


.PHONY: help
help:  ## Show all of tasks
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


.PHONY: watch-build
watch-build:  ## watch build
	RUST_BACKTRACE=1 cargo watch -x 'build'


.PHONY: watch-test
watch-test:  ## watch test task
	RUST_BACKTRACE=1 cargo watch -s '/usr/bin/make test'


.PHONY: watch-test-one
watch-test-one:  ## watch test-one task
	RUST_BACKTRACE=1 cargo watch -s '/usr/bin/make test-one'


.PHONY: watch-bench
watch-bench:  develop-release  ## watch benchmark
	RUSTFLAGS="-C target-cpu=native" cargo watch -s 'poetry run pytest tests -s --benchmark-autosave --benchmark-storage=file:///tmp/.cachers-benchmarks --benchmark-max-time=0.1 --benchmark-min-rounds=2'


.PHONY: watch-bench-compare
watch-bench-compare:  develop-release  ## watch benchmark
	RUSTFLAGS="-C target-cpu=native" cargo watch -s 'poetry run pytest tests --benchmark-autosave --benchmark-storage=file:///tmp/.cachers-benchmarks --benchmark-max-time=0.1 --benchmark-min-rounds=2 --benchmark-compare '


.PHONY: build
build:  ## Builds Rust code and Python modules
	poetry run maturin build


.PHONY: develop
develop:  ## Installs the crate as module in the current virtualenv, see $VIRTUAL_ENV
	poetry run maturin develop


.PHONY: develop-release
develop-release:  ## Installs release built the crate as module in the current virtualenv, see $VIRTUAL_ENV
	RUSTFLAGS="-C target-cpu=native" poetry run maturin develop --release


.PHONY: test-one
test-one:  develop  ## Running tox
	poetry run pytest tests -s


.PHONY: test
test:  develop  ## Running tox on parallel
	poetry run tox --parallel


.PHONY: release-test-one
release-test-one:  develop-release  ## Running tox
	poetry run pytest tests -s


.PHONY: release-test
release-test:  develop-release  ## Running tox on parallel
	poetry run tox --parallel
