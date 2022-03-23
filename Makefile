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
