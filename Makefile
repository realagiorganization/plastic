# Variables users can override: e.g. `make DOCKER_IMAGE=plastic-test all_containers`
DOCKER_IMAGE ?= plastic
DOCKER_VARIANTS ?= ui tui
DOCKER_CONTEXT ?= .

VAGRANT_BOX_NAME ?= plastic-dev
VAGRANT_OUTPUT ?= build/$(VAGRANT_BOX_NAME).box

.PHONY: all_containers docker-images vagrant-image

all_containers: docker-images vagrant-image

docker-images:
	@for variant in $(DOCKER_VARIANTS); do \
		echo "Building Docker image for $$variant variant"; \
		docker build --build-arg DEFAULT_VARIANT=$$variant -t $(DOCKER_IMAGE):$$variant $(DOCKER_CONTEXT); \
	done

vagrant-image:
	@echo "Building Vagrant box $(VAGRANT_BOX_NAME)"
	vagrant up --provision
	mkdir -p $(dir $(VAGRANT_OUTPUT))
	vagrant package --output $(VAGRANT_OUTPUT)
	@echo "Vagrant box written to $(VAGRANT_OUTPUT)"
