CC = gcc -std=gnu99
CFLAGS = -g -O2 -Wall -Wextra
CPP = gcc -std=gnu99 -E
CPPFLAGS = -g

PREFIX=/usr/local

ddate:
	$(CC) $(CCFLAGS) ddate.c -o ddate
	gzip -c ddate.1 >> ddate.1.gz

install:
	mkdir -p $(PREFIX)/bin/
	mkdir -p $(PREFIX)/share/man/man1/
	cp ddate $(PREFIX)/bin/
	cp ddate.1.gz $(PREFIX)/share/man/man1/

.PHONY: clean
clean:
	rm -f ddate ddate.1.gz
