CC=gcc
CFLAGS=-O2 -fPIC
AR=ar

TARGET=libhello.a
OBJS=hello.o

all: $(TARGET)

*.o: *.c
	$(CC) $(CFLAGS) -c $<

$(TARGET): $(OBJS)
	$(AR) rcs $(TARGET) $(OBJS)

clean:
	rm -rf *.o *.a
