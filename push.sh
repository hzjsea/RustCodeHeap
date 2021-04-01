



# delet target
d_target(){

	find . -name "target" | xargs -I {} rm -r {}

}

d_push(){

	git add .
	git commit -am "push by shell"
	git push
}


main(){

	d_target
	d_push
}

main

