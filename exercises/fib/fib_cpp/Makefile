main: main.o fib.o
	g++ -Wall -g -o fib main.o fib.o

main.o: main.cpp fib.h
	g++ -Wall -g -c main.cpp

fib.o: fib.h

clean:
	rm *.o
