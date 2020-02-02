NAME = ghash
WORKDIR = /workdir

.PHONY: p/build
p/build:
	@podman build -t ${NAME} .

.PHONY: init
init:
	@podman run --name ${@} -v .:${WORKDIR} -w ${WORKDIR} -it ${NAME} npm install
	@podman rm ${@}

.PHONY: start
start:
	@podman run --name ${@} -p 8080:8080 -v .:${WORKDIR} -w ${WORKDIR} -it ${NAME} npm start
	@podman rm ${@}

.PHONY: build
build:
	@podman run --name ${@} -v .:${WORKDIR} -w ${WORKDIR} -it ${NAME} npm run build
	@podman rm ${@}

.PHONY: run-bash
run-bash:
	@podman run --name ${@} -v .:${WORKDIR} -w ${WORKDIR} -it ${NAME} bash
	@podman rm ${@}
