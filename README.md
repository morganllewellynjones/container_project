## Author: Morgan Jones

## Mission Statement
The purpose of this project is to create a container runtime service, similar to Docker or Podman, but in Rust. The idea ultimately being that Rust has a higher performance ceiling than Go, and may be able to save energy and resources in long running container instances. In short, my mission statement is exactly the same as the project Railcar, which has been archived. As a student, the other purpose of this project is to familiarize myself with linux tools for managing namespaces, creating and connecting network interfaces, etc.

## State of the Project
This project effectively prototypes the basic concepts of containers on a single container process. It may continue to act as a staging ground for me to test concepts and new ideas.

## How to test this Project
This project can only be tested on Linux. In fact, I've only tested it on Debian 12, but I suspect that any Linux based operating system should do the trick. You will need nftables and ip installed, and you will need some way of pulling container images to test. The create_container.sh script assumes you have docker installed, but you could of course pull an image with podman or any similar service.

Run the following scripts in this order:

Pull a docker image and build a container in your local directory.
```bash
sudo ./create_container.sh docker_image_name
```

Mount the necessary folders into the filesystem and unshare the namespaces, running a container instance of that file system.
```bash
cargo run { -r container_directory_name } [ cmd arg ]...
```

Create the necessary virtual infrastructure to connect the containers network namespace to the internet
```bash
sudo ./connect_network.sh container_directory_name
``` 

Destroy the network infrastructure used to connect the container
```bash
sudo ./teardown_network.sh
```

> [!CAUTION]
> If you want to test this multiple times, make sure to teardown the network before creating a new one. The teardown process will flush your nft tables, if you want to preserve your nft tables, a backup of your ruleset is preserved under nft_backup.
