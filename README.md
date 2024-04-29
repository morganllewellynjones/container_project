##Author: Morgan Jones

##Mission Statement
The goal of this project is to create a program that runs a container, such as one pulled from docker hub. The project is to be written in Rust to take advantage of Rust's exceptional performance ceiling (as opposed to the more conventional container approach using Go). Higher performance could mean economic benefits to using said container runtime environment since container technologies tend to run many simultaneous instances on servers.

##Challenges
- The inclusion of system calls will likely necessitate the use of unsafe Rust, offsetting some of Rust's security advantages.

##Steps to completion
- Create a process using unshare that shares none of its parent namespaces
- Chown to move the process into an image filesystem (tarball)
- Create an image filesystem during execution of the program from a given tarball
- Collect command line arguments from user in a coherent, clean and upgradeable way
- Provide user with the ability to share namespaces with running container in a controlled way
    - In particular provide a method for users to expose ports and to mount filesystems to the container
- When the user exits the container the process should not save state, when they run the program again it does so from the base image
- Benchmark the performance and compare it to a primitive Docker container

##What I am not doing
- This is not attempting to replicate any of Dockers build engine capabilities. But you should be able to build an image using Docker build and then run that image in this container system.
