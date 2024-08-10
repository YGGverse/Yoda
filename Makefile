# Define compiler and flags
CXX = g++
CXXFLAGS = `pkg-config --cflags gtkmm-4.0 glibmm-2.68`
LDFLAGS = `pkg-config --libs gtkmm-4.0 glibmm-2.68`

# Define target executable and source files
TARGET = bin/Yoda
SRCS = src/main.cpp $(wildcard src/app/*.cpp)

OBJS = $(SRCS:.cpp=.o)

# Default target
all: $(TARGET)

# Rule to build the executable
$(TARGET): $(OBJS)
	$(CXX) -o $@ $(OBJS) $(LDFLAGS)

# Rule to build object files from source files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Rule to clean up build files
clean:
	rm -f $(TARGET) $(OBJS)