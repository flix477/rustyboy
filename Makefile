.PHONY: build check lint test
build:
	./scripts/run-on-modified.sh "make build -s"

check:
	./scripts/run-on-modified.sh "make check -s"

lint:
	./scripts/run-on-modified.sh "make lint -s"

test:
	./scripts/run-on-modified.sh "make test -s"
