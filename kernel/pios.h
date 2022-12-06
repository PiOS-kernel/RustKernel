#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


#define HEAP_SIZE 32768

#define MAX_PRIORITY 10

#define STACK_SIZE 4096

void create_task(void (*code)(uint8_t*), uint8_t *args, size_t priority);

size_t get_addr(void);

void kernel_init(uint32_t reload_value);

void task_switch_prologue(void);
