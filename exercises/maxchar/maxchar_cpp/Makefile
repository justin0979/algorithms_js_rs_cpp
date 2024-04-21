main: main.o maxchar.o
	g++ -Wall -g -o maxchar main.o maxchar.o

main.o: 
	g++ -Wall -g -c main.cpp

maxchar.o: maxchar.h

clean: 
	rm *.o
