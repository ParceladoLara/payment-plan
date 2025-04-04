.PHONY: test

test-bmp:
	cargo build --release --features bmp
	protoc --proto_path=./src/ --go_out=./test --go_opt=paths=source_relative --go_opt=Mprotos/plan.proto=test. protos/plan.proto
	cd test && go mod tidy
	cd test && go test -v -run 'TestBMPPlan|TestBMPDownPayment'

test-qitech:
	cargo build --release --features qitech
	protoc --proto_path=./src/ --go_out=./test --go_opt=paths=source_relative --go_opt=Mprotos/plan.proto=test. protos/plan.proto
	cd test && go mod tidy
	cd test && go test -v -run 'TestQiTechPlan|TestQiTechDownPayment'

test:
	make test-bmp
	make test-qitech