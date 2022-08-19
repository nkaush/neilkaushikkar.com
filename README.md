# [neilkaushikkar.com](https://neilkaushikkar.com)

This is my personal website. I built it using Rust and Handlebars and host it as a Docker container in an Azure Container Instance. 

## Local Build

I have provided a Makefile to run the bash scripts to build this website.
Simply run `make` in a bash shell to run the website as a Docker container.
Note that you must have the Docker daemon running in the background.

## Deployment

The container published to Docker Hub targets a `linux/amd64` host since Azure linux hosts do not support `arm64` architectures. 
I am on a M1 (ARM) Mac, so the default images built on my computer are not compatible with Azure hosts, hence the call to `docker buildx` targeting `linux/amd64`.
