
.PHONY: help

help: ## Display this help screen
	@echo
	@echo "Usage: make [target] ..."
	@echo
	@sed -n 's/^\([A-Za-z0-9_.-]*\):.*## \(.*\)$$/\t\1: \2/p' Makefile | sort | column -t -s ':'
	@echo

.PHONY: run

install: ## Install dependencies
	@echo "Installing Tailwind"
	npm install -g tailwindcss

run: ## Develop the application
	@echo "Running the app"
	@tailwindcss -i ./src/index.css -o ./dist/output.css --watch &
	@trunk serve --port 30400 --open
