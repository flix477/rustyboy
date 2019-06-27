.PHONY: build lint test
build:
	./scripts/run-on-modified.sh "make build -s"

lint:
	./scripts/run-on-modified.sh "make lint -s"

test:
	./scripts/run-on-modified.sh "make test -s"
