.PHONY: test

test-simple:
	cargo build --release --features simple
	protoc --proto_path=./src/ --go_out=./test --go_opt=paths=source_relative --go_opt=Mprotos/plan.proto=test. protos/plan.proto
	cd test && go mod tidy
	cd test && go test -v -run 'TestSimplePlan|TestSimpleDownPayment'

test-iterative:
	cargo build --release --features iterative
	protoc --proto_path=./src/ --go_out=./test --go_opt=paths=source_relative --go_opt=Mprotos/plan.proto=test. protos/plan.proto
	cd test && go mod tidy
	cd test && go test -v -run 'TestIterativePlan|TestIterativeDownPayment'

test:
	make test-simple
	make test-iterative