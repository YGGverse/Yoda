# Define compiler and flags
CXX = g++
CXXFLAGS = `pkg-config --cflags gio-2.0 glibmm-2.68 gtkmm-4.0 sqlite3`
LDFLAGS = `pkg-config --libs gio-2.0 glibmm-2.68 gtkmm-4.0 sqlite3`

# Define target executable and source files
TARGET = bin/Yoda
SRCS =  src/main.cpp\
		src/app/browser.cpp\
		src/app/browser/header.cpp\
		src/app/browser/header/menu.cpp\
		src/app/browser/header/tab.cpp\
		src/app/browser/main.cpp\
		src/app/browser/main/tab.cpp\
		src/app/browser/main/tab/page.cpp\
		src/app/browser/main/tab/page/content.cpp\
		src/app/browser/main/tab/page/navbar.cpp\
		src/app/browser/main/tab/page/navbar/base.cpp\
		src/app/browser/main/tab/page/navbar/bookmark.cpp\
		src/app/browser/main/tab/page/navbar/history.cpp\
		src/app/browser/main/tab/page/navbar/history/back.cpp\
		src/app/browser/main/tab/page/navbar/history/forward.cpp\
		src/app/browser/main/tab/page/navbar/request.cpp\
		src/app/browser/main/tab/page/navbar/update.cpp\
		src/app/browser/main/tab/label.cpp\
		src/lib/database.cpp\
		src/lib/database/session.cpp

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