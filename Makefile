docker-build:
	docker build -t ch3f-jordan .

docker-run:
	docker run -p 8000:8000 ch3f-jordan