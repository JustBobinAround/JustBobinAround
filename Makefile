serve: build
	python -m http.server

build: 
	cd ./build_system; cargo run -- convert

new:
	cd ./build_system; cargo run -- new

