# Introductin
This is a generic template about creating a trustless auto card game that runs in the ZKWASM VM and performs settlement onchain. In general, implementing a F
OCG auto card game onchain card game between two players needs some special design to make sure the game is fair. We abstract these special stages into five major steps.

0. A game with a unique Id. This Id could be a incremental counter that increase one every time a game is created.
1. Players commit their cards by hash the card number and the game id.
```
let hash = hash(gameId, cards: &[u64]);
commitment_hash(hash) // This will sends the commitment of players card to the server witout revealing the content of their cards
```
2. Players check whether all the players in the same game have send the commitment to the game server by querying the game server about the commitments state. If all the commitments have been submitted, players start to submit their card content to the server so that the server can simulate the result
```
while(true) {
  let state = query_state();
  if (allCommmitted(state)) {
    break;
  }
}
submitContent()
```

3. When all the players have submitted all their contents the server can simulate the gameplay and conclude a winner of the game. However, although the commitment can only ensures the submitted content is not changed (matchs the commitment before), it can not prevents a user quit the game without submitting his content. So we should alow the game server to finalized the game and give penality. 

4. When the result has ben calculated, the server will generate a ZKProff of the whole process and submit it onchan. The onchain verifier will then verify the proof and perform the real settlement onchain.

# Prepare the server side code
Suppose the global state of the server is a list of games 
```
State {
}
```

# 
