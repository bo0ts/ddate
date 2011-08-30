CC = gcc -std=gnu99
CFLAGS = -g -O2 -Wall -Wextra
CPP = gcc -std=gnu99 -E
CPPFLAGS = -g

ddate:
	$(CC) $(CCFLAGS) ddate.c -o ddate
	gzip -c ddate.1 >> ddate.1.gz

install:
	cp ddate /usr/bin/
	cp ddate.1.gz /usr/share/man/man1/

.PHONY: clean
clean:
	rm -f ddate ddate.1.gz
