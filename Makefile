sentinel:

container: container-build
	docker build -t neilk3/website:latest .
	docker run -it --rm -p 80:80 -p 443:443 --env-file local.env neilk3/website:latest 

container-build: clean
	docker build -t neilk3/website:latest .

publish: clean
	docker buildx create --use
	docker buildx build --push --platform linux/amd64,linux/arm64 --tag neilk3/website:latest .
	docker buildx stop

local: local-build
	cargo run --release --bin template_generator
	cargo run --release --bin webserver

clean:
	rm -rf templates/generated
	find . -name ".DS_Store" -delete

local-build: clean
	cargo build --release

azure-shell:
	az container exec --resource-group RG-Neil-Kaushikkar --name personal-website --exec-command "/bin/sh"

azure-logs:
	az container logs --resource-group RG-Neil-Kaushikkar --name personal-website

heroku-deploy:
	git push heroku master

heroku-shell: 
	heroku run bash -a neilkaushikkar

heroku-logs:
	heroku logs --tail
