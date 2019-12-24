NAME = ghash

.PHONY: build
build:
	@docker-compose run ${NAME} npm run build

.PHONY: start
start:
	@docker-compose run ${NAME} npm start

.PHONY: d/build
d/build:
	@docker-compose build

.PHONY: run/shell
run/shell:
	@docker-compose run ${NAME} sh
