# Define compiler and flags
CXX = g++
CXXFLAGS = `pkg-config --cflags gio-2.0 glibmm-2.68 gtkmm-4.0 pangomm-2.48 sqlite3`
LDFLAGS = `pkg-config --libs gio-2.0 glibmm-2.68 gtkmm-4.0 pangomm-2.48 sqlite3`

# Define target executable and source files
TARGET = bin/Yoda
SRCS =  src/app.cpp\
		src/app/browser.cpp\
		src/app/browser/header.cpp\
		src/app/browser/header/main.cpp\
		src/app/browser/header/main/subtitle.cpp\
		src/app/browser/header/main/title.cpp\
		src/app/browser/header/menu.cpp\
		src/app/browser/header/tab.cpp\
		src/app/browser/main.cpp\
		src/app/browser/main/tab.cpp\
		src/app/browser/main/tab/page.cpp\
		src/app/browser/main/tab/page/content.cpp\
		src/app/browser/main/tab/page/content/text.cpp\
		src/app/browser/main/tab/page/content/text/gemini.cpp\
		src/app/browser/main/tab/page/content/text/gemini/reader.cpp\
		src/app/browser/main/tab/page/content/text/gemini/source.cpp\
		src/app/browser/main/tab/page/content/text/plain.cpp\
		src/app/browser/main/tab/page/content/text/plain/reader.cpp\
		src/app/browser/main/tab/page/navigation.cpp\
		src/app/browser/main/tab/page/navigation/base.cpp\
		src/app/browser/main/tab/page/navigation/bookmark.cpp\
		src/app/browser/main/tab/page/navigation/history.cpp\
		src/app/browser/main/tab/page/navigation/history/back.cpp\
		src/app/browser/main/tab/page/navigation/history/forward.cpp\
		src/app/browser/main/tab/page/navigation/reload.cpp\
		src/app/browser/main/tab/page/navigation/request.cpp\
		src/app/browser/main/tab/label.cpp

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