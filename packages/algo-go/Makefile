SHELL := /usr/bin/env bash

.PHONY: test

help: ## this help output
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

test: ## runs tests
	go test ./...
