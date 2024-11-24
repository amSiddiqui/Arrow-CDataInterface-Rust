#include <stdio.h>
#include <stdint.h>

struct ArrowArray
{
    int64_t length;
    int64_t null_count;
    int64_t offset;
    int64_t n_buffers;
    int64_t n_children;
    const void **buffers;
    struct ArrowArray **children;
    struct ArrowArray *dictionary;
    void (*release)(struct ArrowArray *);
    void *private_data;
};

void export_int32_data(struct ArrowArray *array);

int main()
{
    struct ArrowArray array;
    export_int32_data(&array);

    if (array.buffers != NULL)
    {
        const int32_t *data = (const int32_t *)array.buffers[1];
        for (int i = 0; i < 10 && i < array.length; ++i)
        {
            printf("data[%d] = %d\n", i, data[i]);
        }
    }

    if (array.release != NULL)
    {
        array.release(&array);
    }

    return 0;
}
