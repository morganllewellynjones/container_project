#Usage: ./create_container image_name
#i.e. ./create_container debian -> will create a debian container image in the directory for use

docker create -it --init --name $1 $1
docker export -o $1_container.tar $1
mkdir $1_container
mv $1_container.tar $1_container/
cd $1_container
tar -xf $1_container.tar
rm -f $1_container.tar

docker container rm $1
