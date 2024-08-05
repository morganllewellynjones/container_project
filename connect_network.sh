set -eux

# We need to setup the network for a running container
# The assumption is that before running this script you have created a container using the ./run_container.sh script
if [ "$#" -eq 0 ]; then
	echo "Please provide the name of a container image directory from which to establish a network."
fi

# Provide the container with your hosts and resolve file, necessary for accessing the internet
# In future we should obfuscate your file information, but for now we pass it directly
container=$1
cp /etc/hosts $container/etc/
cp /etc/resolv.conf $container/etc/

# It is very likely in the context of this container project that the most recent unshare process is the container process we wish to target.
# In the future we should integrate this bash script into the rust code. The rust code can accurately track exactly which process we want to target so we don't have to guess.
pid=`pgrep -n unshare`

if [ `cat /proc/sys/net/ipv4/ip_forward` != 1 ]; then
	su -c "echo 1 > /proc/sys/net/ipv4/ip_forward"
fi

# Create named netns for rust container
ip netns attach rc $pid

# Create a veth pair to virtually connect the rc namespace with the host netns
ip link add veth-host type veth peer veth-rc
ip link set veth-rc netns rc

# Bring both veth devices up
ip link set veth-host up
ip netns exec rc ip link set veth-rc up
ip netns exec rc ip link set lo up

# Give each veth an address so that traffic can be routed through them
ip address add 10.0.3.1/24 dev veth-host
ip netns exec rc ip address add 10.0.3.2/24 dev veth-rc

# Make veth-host the default gateway for veth-rc
ip netns exec rc ip route add default via 10.0.3.1

# Get the hosts default gateway interface and store its name
default_gateway_interface=`ip route show default | awk '{print $5}'`

# Store a backup of the nft tables in case the user wants to restore them later
# This is done because the teardown_network.sh script will flush the nft ruleset
nft list ruleset > nft_backup

# Add filter table, and forward traffic between hosts default gateway and the virtual ethernet pair
nft add table filter
nft add chain filter FORWARD '{type filter hook forward priority filter; policy drop;}'
nft insert rule filter FORWARD iifname $default_gateway_interface oifname veth-host accept
nft insert rule filter FORWARD iifname veth-host oifname $default_gateway_interface accept

# Masquerade ip addresses from the rc netns as originating from the host netns when routing through the default gateway
# Allows rust container netns to communicate with the outside world
nft add table nat
nft add chain nat POSTROUTING '{type nat hook postrouting priority srcnat; policy accept;}'
nft add rule nat POSTROUTING oifname $default_gateway_interface iifname veth-host masquerade
