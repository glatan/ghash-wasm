NAME = ghash-wasm
WORKDIR = /workdir

.PHONY: p.build
p.build:
	@podman build -t ${NAME} .

.PHONY: serve
serve:
	-@podman run --name ${@} -p 8080:8080 -v .:${WORKDIR} -w ${WORKDIR} -it ${NAME} trunk serve
	@podman rm ${@}

.PHONY: build.release
build.release:
	-@podman run --name ${@} -v .:${WORKDIR} -w ${WORKDIR} -it ${NAME} trunk build --release
	@podman rm ${@}

.PHONY: run.bash
run.bash:
	-@podman run --name ${@} -v .:${WORKDIR} -w ${WORKDIR} -it ${NAME} bash
	@podman rm ${@}
