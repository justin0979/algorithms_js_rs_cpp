main: main.o chunk.o
	g++ -Wall -g -o chunk main.o chunk.o

main.o: main.cpp chunk.h
	g++ -Wall -g -c main.cpp

chunk.o: chunk.h

clean:
	rm *.o
