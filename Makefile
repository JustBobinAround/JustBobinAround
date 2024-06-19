serve: build
	cd ./build_system; cargo run -- serve 

build: 
	cd ./build_system; cargo run -- convert

new:
	cd ./build_system; cargo run -- new

push: build
	git add .
	git commit -m "running build"
	git push
