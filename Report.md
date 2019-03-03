This is the report for the 2019 Software Architecture Project course.

# Introduction

*CYFighter: Zesty Green Falacious Refuse* is a fighting game I started working on in the September of 2018. For this course I designed the architecture of the input handling for the engine as well as implemented that design partially.

# Requirements

The condenced goal of the game is to be "The Ultimate Customizable Poverty Anime Fighter". A full breakdown of what that means is in the [design.md](design.md). What's relevant to the input handling in that goal are the "Customizable" and "Fighter" parts so I will include their explanations in this chapter as well as additional details on how they affect input handling in particular. After this I will explain some other requirements imposed by the architecture and tooling decisions I've made before the course.

## Customizable
"The engine should highly customizable. Users should be able to add new characters, stages and other assets with relative ease." 

Before the course I made an architectural decision to decouple characters as much possible from the engine code in order to maximize customizablility. In an optimal situation everything related to a character, including their behaviour, would come from an asset package that would be loaded at the startup of the game or perhaps even while the game is running. This would mean that you can develop characters without touching the engine code at all.

Since characters and input handling are very intertwined, this means that input handling needs to fit well with the decoupled character architecture. More specifically there needs to be a way to configure character specific inputs. Not all characters are going to use the same inputs and it is possible that a character developer could come up with a non-standard input which they need to be able to implement.

## Anime Fighter

The game is going to be a 2d fighting game. This means that the input handling should be able to handle inputs typical to that genre. In most other genres single inputs map directly to character actions. In fighting games however an action for a given input can be different depending on what inputs were received in the past. This make input handling more challenging especially when there are some corner cases of input handler in the past that have become standard inputs in fighting games.

## Other Requirements

The overall architechture of the engine is centered around the [ECS](https://en.wikipedia.org/wiki/Entity_component_system) pattern using the Specs Parallel ECS library. ECS generally well suited for videogames and I'm fairly comfortable with it because I have previous experience with specs. In ECS all logic is divided into systems that have different responsibilities.