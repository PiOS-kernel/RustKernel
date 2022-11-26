#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


#define HEAP_SIZE 32768

#define HEAP_START 33587200

#define MAX_PRIORITY 10

void create_task(void (*code)(uint8_t*), uint8_t *args, uint8_t priority);
