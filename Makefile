deploy:
	git push heroku master

local:
	cargo run --release

clean:
	cargo clean

shell: 
	heroku run bash -a neilkaushikkar

logs:
	heroku logs --tail

container: 
	docker build -t neilk3/website:latest .
	docker run -it --rm -p 80:80 neilk3/website:latest 

publish: 
	docker buildx build --platform=linux/amd64 -t neilk3/website:latest .
	docker push neilk3/website:latest