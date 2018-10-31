This file tries to document the various design choices and the reasoning behind them. If there's something unclear about the design of the game and its code and this file did not help to clarify it, please create an issue for it.

Game Design
===========

Goals
-----

The condenced goal of the game is to be "The Ultimate Customizable Poverty Anime Fighter". Lets break that up to see what that actually means and how it affects the design goals.

### Ultimate

The game should be the best of its kind. Its what everyone goes to when they want to play a customizable poverty anime fighter. Doesn't hurt to have ambition. How this will be achieved is just that extra care will be put to achieve the other goals.

### Poverty

The game should be accessible to people of all kinds of monetary situations. That goal is already partially achieved since the code is available openly under MIT licence. I considered a GPL licence since that would keep all modifications free(libre) but a GPL licence would probably make an already unlikely console version even harder and that would interfere with the poverty goal and the ultimate goal.    
Even if the game is available for free it still might not be accessible if it only runs on high-end hardware. Therefore the game should be able to run on a wide range of hardware.

### Customizable

### Anime Fighter

It's a 2d anime fighting game. This means that to players that have played games such as melty blood, guilty gear, blazblue or under night in-birth the game should have a familiar feel to it. Another

Architecture
============