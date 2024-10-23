# PAYMENT PLAN
This is the Lara Payment Plan, the heart of Lara Credit Proposal System. It is a binary that calculates the payment plan for a given credit proposal.

# Package Structure
The package is structured as follows:
- `core`: Contains the core logic of the payment plan
- `node`: Contains the [Neon](https://neon-rs.dev) for the NodeJs wrapper of the binary.
- `bin`: Contains the binary and the protobuf specification for any communication between the binary and the language that uses it.
- `wasm`: Contains the wasm for the payment plan.

If you want to see more about each package, you can see their individual MD files.

- [core](docs/core.md)
- [node](docs/node.md)
- [bin](docs/bin.md)
- [wasm](docs/wasm.md)

And if you have no knowledge of Rust, you can see how rust project are structured [here](docs/rust.md)


# How to build

To build any of the binaries, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.

## NodeJs

With NodeJs installed, you can build the NodeJs wrapper by running the following commands:

```bash
cd node
npm run build:release
```

This will generate a `index.node` file in the `node` directory.

## Binary

For the binary, you will need protoc installed.

```bash	
sudo apt update
sudo apt install -y protobuf-compiler
```

Set the variable `PROTOC` on your bashrc or zshrc to the path of the protoc binary.

```bash
sudo nano ~/.bashrc
export PROTOC=/usr/bin/protoc
source ~/.bashrc
```

Then you can build the binary by running the following commands:

```bash
cargo build --package payment-plan --release
```

This will generate a binary called `payment-plan` in the `target/release` directory.

## WASM

First install [wasm-pack](https://github.com/rustwasm/wasm-pack?tab=readme-ov-file)

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Then you can build the wasm by running the following commands:

```bash
cd wasm
wasm-pack build --target web  
```

And then to run the example um can run with a simple server like so:

```bash
python3 -m http.server --directory example
```

Then you can open your browser and go to `http://localhost:8000` to see the example.

# Usage
First create a synlink to the binary in your project.

```bash
sudo ln -s ~/path/to/your/project/target/release/payment-plan /usr/local/bin/payment-plan
```
Now you can call the binary from your project without having to specify the full path.

On docker, you can add the binary to the container by adding the following line to your Dockerfile.

```Dockerfile
COPY --from=builder /path/to/your/project/bin/payment-plan /usr/local/bin/payment-plan
```

## NodeJs

Start by coping the `index.node` file to your project.

```bash
cp node/index.node /path/to/your/project
```

Then you can use it in your project like so:

```typescript
import * as funcs from 'path/to/your/project/index.node';
//you can type the functions to get intellisense
const { calculatePlan } = funcs as PaymentPlanFunctions; 

export { calculatePlan };
```

typescript will not recognize the .node file so you will need to create a declaration file for it on the root of your project.

```typescript
//declarations.d.ts
declare module '*.node' {
  const content: any;
  export = content;
}
```

and include it in your tsconfig.json

```json
{
  "include": ["declarations.d.ts"]
}
```

## Go
Assuming that the synlink has already been created, and/or your Docker image has the line to copy the binary to the bin directory.

You will need the protoc-gen-go plugin to generate the Go code from the protobuf file.

```bash
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
```
Add the Go binary path to your system's PATH

```bash
sudo nano ~/.bashrc
export PATH=$PATH:$(go env GOPATH)/bin
source ~/.bashrc
```

Then you can generate the Go code by running the following command:

```bash
protoc --proto_path=/path/to/payment-plan/bin/src --go_out=. --go_opt=paths=source_relative protos/plan.proto
```

This will generate a `plan.pb.go` file in the root of your project.

Then you can use the binary in your Go project like so:

```go
func main() {
  // Initialize your PlanParams message
	params := &protos.PlanParams{}

	// Serialize PlanParams to bytes
	data, err := proto.Marshal(params)
	if err != nil {
		log.Fatalf("Failed to serialize PlanParams: %v", err)
	}

	// Prepare the command you want to execute
	cmd := exec.Command("payment-plan") 

	// Create a bytes buffer to hold the serialized data
	var in bytes.Buffer
	var out bytes.Buffer
	in.Write(data)
	cmd.Stdin = &in
	cmd.Stdout = &out

	// Execute the command
	err = cmd.Run()
	if err != nil {
		log.Fatalf("Failed to execute CLI command: %v", err)
	}

	// Deserialize the response from the command
	var plan protos.PlanResponses
	err = proto.Unmarshal(out.Bytes(), &plan)
	if err != nil {
		log.Fatalf("Failed to deserialize PlanResponses: %v", err)
	}
}
```

# Testing
Assuming that you already have the setup for every package, you can run the tests by running the following command on the root of the project:

```bash
make test
```