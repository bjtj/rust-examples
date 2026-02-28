CC=gcc
CFLAGS=-O2 -fPIC

TARGET=libhello_shared.so
OBJS=hello.o

all: $(TARGET)

*.o: *.c
	$(CC) $(CFLAGS) -c $<

$(TARGET): $(OBJS)
	$(CC) -shared -o $(TARGET) $(OBJS)

clean:
	rm -rf *.o *.a *.so
