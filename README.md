# simple-server

This is a simple server implemented by Rust.
You can choose framework libraries from which are [Rocket](https://rocket.rs/) or [actix-web](https://actix.rs/) to run.


| Library   | PackageName |
| --------- | ----------- |
| Rocket    | by-rocket   |
| actix-web | by-actix    |

## How to run
You can run this server from Cargo or Docker.  

If you run it implemented by Rocket from Cargo. You type the following command.  
```sh
cargo run -p by-rocket
```
Option `-p by-rocket` specify which package name have you chosen.
In this example, you have chosen Rocket. Therefore you type `by-rocket` as the package name, according to preceding table.  


And if you run it implemented by actix-web from Docker. You type the following command.  
```sh
docker build -t simple-server:actix --build-arg framework=by-actix
```
Optioin `--build-arg framework=by-actix` specify which package name have you chosen.
In this example, you have chosen actix-web. Therefore you type `by-actix` as the package name, according to preceding table.
