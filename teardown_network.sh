# Removing the veth-host also removes the paired veth-rc device
ip link delete veth-host

# Remove the rust container network namespace
ip netns delete rc

# Flush the nft ruleset
nft flush ruleset

# Restore the previous nft ruleset
nft -f nft_backup
rm -f nft_backup
