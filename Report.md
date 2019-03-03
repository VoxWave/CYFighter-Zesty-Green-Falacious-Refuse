This is the report for the 2019 Software Architecture Project course.

# Introduction

*CYFighter: Zesty Green Falacious Refuse* is a fighting game I started working on in the September of 2018. For this course I designed the architecture of the input handling for the engine as well as implemented that design partially.

# Requirements

The condenced goal of the game is to be "The Ultimate Customizable Poverty Anime Fighter". A full breakdown of what that means is in the [design.md](design.md) file. What's relevant to input handling in that goal are the "Customizable" and "Fighter" parts so I will include their explanations in this chapter as well as additional details on how they affect input handling in particular. After this I will explain some other requirements imposed by the architecture and tooling decisions I've made before the course.

## Customizable
"The engine should highly customizable. Users should be able to add new characters, stages and other assets with relative ease." 

Before the course I made an architectural decision to decouple characters as much possible from the engine code in order to maximize customizablility. In an optimal situation everything related to a character, including their behaviour, would come from an asset package that would be loaded at the startup of the game or perhaps even while the game is running. This would mean that you can develop characters without touching the engine code at all. The non-engine side code should not however be responsible for anything else than what is necessary to implement characters and their functionality. In other words if its possible to implement in the engine without critically affecting customizablity then it should be implemented in the engine.

Since characters and input handling are very intertwined, this means that input handling needs to fit well with the decoupled character architecture. More specifically there needs to be a way to configure character specific inputs. Not all characters are going to use the same inputs and it is possible that a character developer could come up with a non-standard input which they need to be able to implement.

## Anime Fighter

The game is going to be a 2d fighting game. This means that the input handling should be able to handle inputs typical to that genre. In most other genres single inputs map directly to character actions. In fighting games however an action for a given input can be different depending on what inputs were received in the past. This make input handling more challenging especially when there are some corner cases of input handlers in the past that have become standard inputs in fighting games.

## Other Requirements

The overall architechture of the engine is centered around the [ECS](https://en.wikipedia.org/wiki/Entity_component_system) pattern using the Specs Parallel ECS library. ECS generally well suited for videogames and I'm fairly comfortable with it because I have previous experience with specs. 

I've also recently switched to using [Amethyst](https://www.amethyst.rs/) which is a game engine library built on top of specs. I mainly wanted to replace [piston-window](https://github.com/PistonDevelopers/piston_window), which I was using for rendering, because I was running into performance issues in another project and I was generally dissatisfied with its API quality and documentation. Amethyst is also data-oriented/data-driven in its design which means it has tooling which will help me achieve the Customizability goal.

These two existing architectural choices affect the implementation of the input handling. Mainly the input handling should mesh well with the ECS architecture. It should also utilize amethyst as much as possible because there's no reason to implement something that is already available through amethyst.

# Design

The general story of inputs in fighting games that a player presses a button and then their character does something. What happens first after the button press that the operating system receives that input and through its APIs or an external library our program recieves the input. The program can also recieve inputs that are not relevant to the game. This is why these "raw" inputs need to be mapped into logical inputs that the game logic can use. This is the first phase of my input handling design. The second part is what is done with the logical inputs.

## Phase 1

Turns out amethyst can handle most of the first phase. In amethyst one can write a configuration file which contains axes and actions and the inputs that correspond to them. An axis represent an input type that can be represented by a float value between -1 and 1. In a gamepad axes would represent analog sticks as well as some types of triggers. Actions represent inputs that can only be on or off. In a gamepad this means all of the buttons. Actions can also be bound to combinations of inputs. This type of action is on when all of the buttons it consists of are pressed. For this game there will be four axes and 10 actions defined. 5 buttons and 2 directional input axes for each player.

Once the configuration file is created it can be loaded into an input bundle and that input bundle can be added to the game data of amethyst. Adding an input bundle also adds an System to the game which updates the states of the axes and actions. These states can then be accessed in any System through an `InputHandler<String, String>` resource.

As was already said, the first phase of input handling is almost completely handled by amethyst but there is one little quirk of fighting games that needs to be handled manually. The Fighting Game genre originated from the arcades so the input method for which they were originally designed was an arcade stick and buttons. Arcade sticks are more primitive than modern analog sticks. They don't track on what point they are on the vertical and horizontal axes and instead just have switches that turn on when you tilt the stick enough towards a direction on an axis. This leads to the fact that there are only 9 states that an arcade stick can be in. These are the cardinal and diagonal directions and the neutral position.

What this means is that the axis inputs need to be converted to the more absolute inputs that represent the positions of a arcade stick. Luckily the conversion is fairly simple. Figure out which values between -1 and 1 in one axis correspond to the two extremes and the neutral position. Basically the axis is in the neutral position from 0 up until it reaches a threshold in either direction after which it is in the position of that direction. This conversion will done be in an separate System which will then send the inputs onwards to the next System. This system will also keep track of the states of the stick and buttons and will only send out inputs to phase 2 when a change of state happens. This makes parts of phase 2 easier simply because there will be less logical inputs that need to be handled.

## Phase 2

The second phase of the input handling which also the more complicated one is the input parsing phase. As was already mentioned in the [Requirements](#Requirements) chapter, in fighting games characters do actions depending not only on a single input but also on combinations of inputs. This means that inputs need to be buffered up to recognize larger patterns. A parser would then look at the buffer and try to parse a commands which it would send to the system that handles character logic. The buffer also needs to empty out inputs which are too old because we don't want end up parsing accidentally buffered inputs.

Technically the parser (and the buffer) could be implemented outside the engine by the character developer but because there's a fair amount of standard commands in fighting games, it would be unreasonable to implement parsers for such commands from scratch every time a character needs them. This is why I decided that I would implement parsing of the most common commands in the engine code. However design I have chosen, as you will see, does also support parsing everything outside of the engine if there is a need for that.

The [Customizability](#Customizable) requirement adds some contraints on the parser design. Mainly I cannot just implement a big parser that parses all of the commands and then send those parsed commands to the character system. This is because not every character is going to use every command. That approach could work but it'd unnecessarily increase the workload of the character developer. This is because for every missing command, a character developer would need to figure out if that inputs for that command contain another smaller command that the character does support.

Originally I thought of using [parser combinators](https://en.wikipedia.org/wiki/Parser_combinator) to solve this problem. With combinatory parsing one first creates small parsers that parse really simple things like recognizing a single input. Then using a parser combinator, these small parsers are combined into a bigger parsers. These bigger parsers can then be combined again and this process is repeated until the final parser has been constructed. Using this pattern I would have first implemented parsers for all of the inputs and then at run time a config made by the character developer would dictate which of these parsers would be used and in which order and the engine would then dynamically create the final parser for that character.

There were several problems with this approach. The main problem is that in rust the combinators are implemented using macros (at least in [nom](https://docs.rs/nom/4.2.1/nom/)). [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html) are expanded at compile time so the final combinator would have to implemented in a more manual fashion. This combined with the fact that I'm not very comfortable with the pattern yet in general and that I was running into trouble before even getting to implement any parsers lead me to abandon the idea. I would have had to either implement a bunch of [traits](https://doc.rust-lang.org/book/ch10-02-traits.html) for nom or implement my own combinator macros before I could even get started and I would have to do the final combination manually anyway.

The solution I ended up replacing the combinator idea with does borrow some aspects from the combinator idea. I will implement individual parser functions for all of the commands and then using a configuration file I will figure out which will be tried for the character and in what order. The parser functions are just normal functions that take in a reference of the buffer and return an [Optional](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values) command.

The parser functions do not look for the pattern that they looking for in the whole buffer. Instead they only try to match their pattern starting from the newest input meaning if the latest input cannot be a part of the pattern they are looking for they will automatically fail. This is because we try to parse a single command every time an input is added. Most of the time the new input is an directional so we output a direction command because we want the characters to move as soon as possible. If the new input is a button press then we check for the largest matching command which contains that button and output that command to the character system.

Tk inputs are one of primary reason for this way of doing parsing to so I will explain what they are. A tk input is an input where after inputting an motion and before pressing a button the player moves the stick position into an upper corner. This will cause the character first jump because of that upper corner input but then do an aerial version of that command immediately after that because the motion was buffered just before the jump. You can see that this design supports TK's.

It was mentioned earlier that a character developer could implement their own parser. If the characters input configuration does not enable any of the parsers other than the directional and button parsers then this effectively result in the logical inputs just being forwarded into the character logic which the character developer can then use implement their custom parsers.

# Implementation

The design is not implemented in its full form yet because of time constraints and the fact that the whole game is not implemented yet. However the most important parts of it are ready enough in this proof of concept that I can comment on the effectiveness of the design. This POC program does not have any gameplay. Instead it (crudely) visualizes a fightstick. It also parses commands which can be seen in the console output. Currently only the directional input and one button are in use and the supported commands are the hadouken, button and direction commands.

The relevant files to look into in terms of input architecture are [binding_config.ron](assets/configs/binding_config.ron), [fight_stick.rs](src/fight_stick.rs) and [parser.rs](src/parser.rs). 