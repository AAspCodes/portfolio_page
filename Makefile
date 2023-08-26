build-actions-image:
	sudo docker build -t aaspcodes/action-rust-trunk:latest -f actions/Dockerfile .
	docker push aaspcodes/action-rust-trunk:latest