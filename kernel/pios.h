#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


#define HEAP_SIZE 32768

#define MAX_PRIORITY 10

extern const uint32_t HEAP_MEMORY;

void create_task(void (*code)(uint8_t*), uint8_t *args, uint8_t priority);

uint32_t get_heap_addr(void);

void heap_init_wrapper(size_t start_addr, size_t size);

void kernel_init(size_t heap_start, uint32_t reload_value);

void prova(void);
