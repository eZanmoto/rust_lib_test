// Copyright 2017 Sean Kelleher. All rights reserved.
// Use of this source code is governed by a BSD
// licence that can be found in the LICENCE file.

#include "stdio.h"

#include "print.h"

int print_all(int n, char **s) {
    int i = 0;
    for (i = 0; i < n; i++) {
        printf("%s ", s[i]);
    }
    printf("\n");
    return 0;
}
