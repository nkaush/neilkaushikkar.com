deploy:
	git push heroku master

local:
	cargo run --release

azure-local:
	sh azure-scripts/local-start.sh

azure-deploy:
	sh azure-scripts/deploy.sh

clean:
	cargo clean

shell: 
	heroku run bash -a neilkaushikkar

logs:
	heroku logs --tail
