## Author: Morgan Jones

## Mission Statement
The purpose of this project is to create a container runtime service, similar to Docker or Podman, but in Rust. The idea ultimately being that Rust has a higher performance ceiling than Go, and may be able to save energy and resources in long running container instances. In short, my mission statement is exactly the same as the project Railcar, which has been archived. As a student, the other purpose of this project is to familiarize myself with linux tools for managing namespaces, creating and connecting network interfaces, etc.

## State of the Project
This project is in its infancy, but it effectively prototypes the basic concepts of containers on a single container process. It has no system daemon for managing multiple containers.

## How to test this Project
Run the following scripts in this order:

Pull a docker image and build a container in your local directory.
```bash
./create_container.sh docker_image_name
```

Mount the necessary folders into the filesystem and unshare the namespaces, running a container instance of that file system.
```bash
./run_container.sh { -r container_directory_name } [ cmd arg ]...
```

Create the necessary virtual infrastructure to connect the containers network namespace to the internet
```bash
./connect_network.sh container_directory_name
``` 

Destroy the network infrastructure used to connect the container
```bash
./teardown_network.sh
```

> [!CAUTION]
> It is necessary to perform the network teardown step before you connect a new container network. The teardown process will destroy the network namespace used when connecting a new network, additionally it will flush your nft tables. If you want to preserve your nft tables, a backup of your ruleset is preserved under /data/nft_backup.
