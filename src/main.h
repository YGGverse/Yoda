#pragma once

#include <iostream>
#include <gtk/gtk.h>

#include "app/browser.h"

void static activate(
    GtkApplication *application
);

int main(
    int argc,
    char *argv[]
);