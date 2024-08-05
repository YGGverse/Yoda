# Define compiler and flags
CXX = g++
CXXFLAGS = `pkg-config --cflags gtk4`
LDFLAGS = `pkg-config --libs gtk4`

# Define target executable and source files
TARGET = bin/Yoda
SRCS =  src/main.cpp\
		src/app/browser.cpp\
		src/app/browser/container.cpp\
		src/app/browser/container/page.cpp\
		src/app/browser/container/tab.cpp\
		src/app/browser/header.cpp\
		src/app/browser/header/bar.cpp\
		src/app/browser/header/bar/title.cpp\
		src/app/browser/menu.cpp

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