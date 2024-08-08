# Define compiler and flags
CXX = g++
CXXFLAGS = `pkg-config --cflags gtkmm-4.0`
LDFLAGS = `pkg-config --libs gtkmm-4.0`

# Define target executable and source files
TARGET = bin/Yoda
SRCS =  src/main.cpp src/app/browser.cpp
#		src/app/browser.cpp\
#		src/app/browser/container.cpp\
#		src/app/browser/container/page.cpp\
#		src/app/browser/container/tab.cpp\
#		src/app/browser/header.cpp\
#		src/app/browser/header/tab.cpp\
#		src/app/browser/header/menu.cpp\
#		src/app/browser/header/menu/main.cpp\
#		src/app/browser/header/menu/main/tab.cpp\
#		src/app/browser/header/menu/main/tab/append.cpp\
#		src/app/browser/header/menu/main/debug.cpp\
#		src/app/browser/header/menu/main/quit.cpp

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