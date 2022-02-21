<!--  Run the code -->
cargo run -- --attribute parents --id 10 --len yes --name yes
cargo run -- --attribute children --id 10 --len yes --name yes
cargo test
<!-- Args -->
Run the code with  Args :
--attribute parents|children 
--id 7 
--len yes 
--name yes
<!-- Todo list -->
Cahier des charges
	Récupérer certaines informations contenu dans le fichier list.json en Rust et faire les tests unitaires

		1 - Le nombre d'enfants direct d'un objet par id
			ex cmd: children --fromId=0 --len=yes
				ouput: length is 2

		2 - Les noms et nombre d'enfants direct d'un objet par id
			ex cmd: children --fromId=0 --len=yes --name=yes
				ouput: length is 2 and names are b, c

		3 - Les noms de tout les parents direct et indirect d'un objet par id
			ex cmd: parents --fromId=7 --len=yes --name=yes
				ouput: length is 5 and names are g, e, d, b, a   