synth_cdk : build build_cdk
	cd point6_deploy && cdk synth

deploy : build build_cdk
	cd point6_deploy && cdk deploy

build_cdk : 
	cd point6_deploy && npm run build

build : compile slack_incoming_handler big_hero_echo

compile: 
	cargo build --target x86_64-unknown-linux-musl --release;

big_hero_echo : compile
	cd target/x86_64-unknown-linux-musl/release/ && rm -f bootstrap && cp -f big_hero_echo bootstrap && zip big_hero_echo.zip bootstrap

slack_incoming_handler : compile
	cd target/x86_64-unknown-linux-musl/release/ && rm -f bootstrap && cp -f big_hero_point6 bootstrap && zip slack_incoming_handler.zip bootstrap

clean :
	cargo clean
