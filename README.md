# The Crab Game Server Built in Rust 
Final Group Submission for CS128H Project

# Team Name
The Crab Game

# Team Members
Kenny Kim (kk67)
Justin Kim (jwk8)

# Project Introduction
A variation on the classic Dinosaur Game (Chrome Dino) that pops up on your browser when there is no internet connection. Instead of a boring old dinosaur, you will be playing as Ferris the Rustacean Crab to survive as long as possible against one other competitor in a online multiplayer server fully developed in Rust. 

# System Overview
- Client
  - Connecting to the server alongside another device
  - Sending jump and duck inputs for the game
  - Displaying the Game
- Server
  - Developing a TCP protocol to ensure strong connection
  - Multithreading
  - Etc.
- UI/UX
  - Ensure the UI design is pretty and easy to look at
  - Fun to play
- Functioning Game
  - Creating a Dinosaur copy that functions

# Possible Challenges
- Creating a Game as we have no prior experience in Game Dev
- Poor Connections
- Bad thread management causing slow gameplay and lag
- Combining the components together

# References
- https://github.com/MarkintoshZ/rust-tic-tac-toe-server/blob/master/README.md#system-overview
- https://sunjay.dev/learn-game-dev/getting-started.html
- https://www.section.io/engineering-education/how-to-build-a-client-server-application-using-rust/
