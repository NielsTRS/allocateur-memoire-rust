//
//  sim-alloc.h
//  SIM-ALLOC-M1
//
//  Created by Vania Marangozova-Martin on 18/09/2020.
//  Copyright © 2020 Vania Marangozova-Martin. All rights reserved.
//

#ifndef sim_alloc_h
#define sim_alloc_h
#include <getopt.h>

void get_the_opts(int argc, char** argv);
void usage(const char *commande);
int check_sim_state(void);

typedef enum strategy {
    FF,
    BF,
    WF
} strategy_t;

extern strategy_t STRATEGY;

extern size_t MEMORY_SIZE;
extern int NB_BLOCKS;
extern size_t BB_SIZE;
extern size_t FB_SIZE;
extern size_t HEADER_SIZE;
extern size_t ALIGN_SIZE;       //default value is no alignement

#endif /* sim_alloc_h */
