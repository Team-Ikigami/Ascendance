## Hello contributor

This is the repository for the ASCENDANCE game. A souls-like RPG focused on player interchangability. This game will most likely not ever be properly finished to the goal of its lead developers, Ikigami and DuckEater54.

### Subsections

- [File meaning](#file-definitions)

## File Definitions

###### This section is to explain the usage of each file and to link to those files.

- [src/actor.rs](src/actor.rs)
- [src/character.rs](src/character.rs)
- [src/inventory.rs](src/inventory.rs)
- [src/config.rs](src/config.rs)
- [src/level.rs](src/level.rs)
- [src/lib.rs](src/lib.rs)
- [src/light.rs](src/light.rs)
- [src/loading_screen.rs](src/loading_screen.rs)
- [src/message.rs](src/message.rs)
- [src/pause_menu.rs](src/pause_menu.rs)
- [src/save_load.rs](src/save_load.rs)
- [src/sound.rs](src/sound.rs)
- [src/bot/mod.rs](src/bot/mod.rs)
	- [src/bot/lower_body.rs](src/bot/lower_body.rs)
	- [src/bot/upper_body.rs](src/bot/upper_body.rs)
	- [src/bot/behaviour/mod.rs](src/bot/behaviour/mod.rs)\
		This file is a work in progress.\
		It is used to link the other files in this folder to what can be compiled. It also has several other functions and classes.\
		To be documented
		
		- [src/bot/behaviour/aim.rs](src/bot/behaviour/aim.rs)\
			This file is a work in progress.\
			It is used to define the behaviours that happen when npe is aiming itself to something.\
			To be documented

		- [src/bot/behaviour/death.rs](src/bot/behaviour/death.rs)\
			This file is a work in progress.\
			It is used to define the behaviour of bots that occurs upon death.\
			To be documented

		- [src/bot/behaviour/find.rs](src/bot/behaviour/find.rs)
		- [src/bot/behaviour/melee.rs](src/bot/behaviour/melee.rs)
		- [src/bot/behaviour/movement.rs](src/bot/behaviour/movement.rs)
		- [src/bot/behaviour/shoot.rs](src/bot/behaviour/shoot.rs)
		- [src/bot/behaviour/threat.rs](src/bot/behaviour/threat.rs)
- [src/door/mod.rs](src/door/mod.rs)
	- [src/door/ui.rs](src/door/ui.rs)
- [src/entitygen/*.rs](src/entitygen/mod.rs)
- [src/items/mod.rs](src/items/mod.rs)
	- [src/items/definition.rs](src/items/definition.rs)
	- [src/items/apparel/mod.rs](src/items/apparel/mod.rs)
		- [src/items/apparel/definition.rs](src/items/apparel/definition.rs)
	- [src/items/weapon/mod.rs](src/items/weapon/mod.rs)
		- [src/items/weapon/bash.rs](src/items/weapon/bash.rs)
		- [src/items/weapon/definition.rs](src/items/weapon/definition.rs)
		- [src/items/weapon/projectile.rs](src/items/weapon/projectile.rs)
		- [src/items/weapon/sight.ea](src/items/weapon/sight.rs)
		- [src/items/weapon/slice.rs](src/items/weapon/slice.rs)
		- [src/items/weapon/thrust.rs](src/items/weapon/thrust.rs)
- [src/player/mod.rs](src/player/mod.rs)
	- [src/player/camera.rs](src/player/camera.rs)
- [src/ui/mod.rs](src/ui/mod.rs)
	- [src/ui/craftingui.rs](src/ui/craftingui.rs)
	- [src/ui/gameui.rs](src/ui/gameui.rs)
	- [src/ui/playerui.rs](src/ui/playerui.rs)
	- [src/ui/serverui.rs](src/ui/serverui.rs)
	- [src/ui/start_menu_ui.rs](src/ui/start_menu_ui.rs)
- [src/utils/mod.rs](src/utils/mod.rs)
	- [src/utils/model_map.rs](src/utils/model_map.rs)
	    


## Level Tree explanitory view

All different perk names are different perks, all same perk names are interconnected ones. 

Example being base.str_1.end_1 connects to base.int_1.nrgitm_1


- Base
	- str_1
		- end_1
			- nrgitm_1
		- hpitm_1
		    - end_2
	- dex_1
		- end_2
			- hpitm_1
		- hpitm_2
	- int_1
		- end_3,
		- nrgitm_1
			- end_1
	- fth_1
		- end_4,
		- nrgitm_2
