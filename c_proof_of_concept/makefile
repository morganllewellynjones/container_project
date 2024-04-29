all: wrapper run clean

wrapper: wrapper.c
	gcc wrapper.c -o wrapper

run: wrapper
	sudo ./wrapper &

clean:
	rm -f wrapper
