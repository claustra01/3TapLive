up:
	docker compose up -d

down:
	docker compose down

restart:
	docker compose down
	docker compose up -d --build

migration:
	docker exec -it tyranno-server bash -c "cargo install diesel_cli"
	docker exec -it tyranno-server bash -c "diesel migration run"	

deploy:
	docker build -t tyranno-server ./server
	docker tag tyranno-server tyranno.azurecr.io/server
	docker push tyranno.azurecr.io/server