# Introductin
This is a generic template about creating a trustless auto card game that runs in the ZKWASM VM and performs settlement onchain. In general, implementing an PVE game between players and the server needs some special design to make sure the game is fair. We abstract these special stages into 4 stages

0. Admin generates a random seed and commit the seed into the state by commitment = hash(seed)
1. Player sign the commitment of the random seed and provide a signature SR
2. Admin reveal the seed by providing the seed to the server
3. Server check the seed has correct hash by hash(seed) = commitment
4. A random number is then generated for the Player by seed xor SR
