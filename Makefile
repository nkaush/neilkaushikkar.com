heroku-deploy:
	git push heroku master

local:
	cargo run --release

clean:
	cargo clean

heroku-shell: 
	heroku run bash -a neilkaushikkar

azure-shell:
	az container exec --resource-group RG-Neil-Kaushikkar --name personal-website --exec-command "/bin/sh"

heroku-logs:
	heroku logs --tail

container: 
	docker build -t neilk3/website:latest .
	docker run -it --rm -p 80:80 -p 443:443 --env-file local.env neilk3/website:latest 

publish: 
	docker buildx build --push --platform linux/arm64/v8,linux/amd64 --tag neilk3/website:latest .