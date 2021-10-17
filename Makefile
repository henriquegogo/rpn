CFLAGS?=-Wpedantic -Wall -Wextra

all:
	$(CC) $(CFLAGS) rpn.c -o rpn
		
clean:
	rm rpn
