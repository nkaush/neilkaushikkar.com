# neilkaushikkar.com

This is my personal website. I built it using Rust and Handlebars and host it using an Azure Functions app.

## Local Build

I have provided a Makefile to run the bash scripts to build this website. 
You will need the Azure Functions Core tools to run this project locally.

Simply run `make local` in a bash shell to run the project.

## Deployment

Run `make deploy` in a shell to deploy the project. Only I can run this as this requires Azure credentials to complete.
